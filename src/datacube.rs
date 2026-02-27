//! Datacube API for Article Statistics and Analysis.
//!
//! This module provides methods to fetch statistical data about articles
//! such as reads, shares, detailed statistics, and summary overviews.

use crate::auth::TokenManager;
use crate::error::Result;
use crate::http::{WeChatHttpClient, WeChatResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::debug;

/// Represents a general Datacube request for a specific date range.
#[derive(Debug, Serialize)]
pub struct DatacubeRequest {
    /// Start date (YYYY-MM-DD)
    pub begin_date: String,
    /// End date (YYYY-MM-DD)
    pub end_date: String,
}

/// Generic wrapper for Datacube API responses.
#[derive(Debug, Deserialize, Serialize)]
pub struct DatacubeResponse<T> {
    /// List of data items returned
    pub list: Vec<T>,
    /// Whether the data is delayed (false means data is fresh)
    #[serde(default)]
    pub is_delay: bool,
}

// ============== getarticleread (Daily Read Stats) ==============

/// Represents the read statistics for an article on a given date.
#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleReadTotal {
    /// Date of statistics (YYYY-MM-DD)
    pub ref_date: String,
    /// Message ID combined with the index, e.g. "12003_3"
    pub msgid: String,
    /// Detailed statistics for this article
    pub detail: ArticleReadDetail,
}

/// Detailed read statistics for an article.
#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleReadDetail {
    /// Total read users
    pub read_user: u32,
    /// Breakdown of readers by source
    pub read_user_source: Vec<ReadUserSource>,
}

/// Breakdown of reading users by source.
#[derive(Debug, Deserialize, Serialize)]
pub struct ReadUserSource {
    /// Number of users from this source
    pub user_count: u32,
    /// Source description (e.g., "全部", "公众号消息", "朋友圈", etc.)
    pub scene_desc: String,
}

// ============== getarticleshare (Daily Share Stats) ==============

/// Represents the share statistics for an article on a given date.
#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleShareTotal {
    /// Date of statistics (YYYY-MM-DD)
    pub ref_date: String,
    /// Message ID combined with the index, e.g. "12003_3"
    pub msgid: String,
    /// Detailed share statistics
    pub detail: ArticleShareDetail,
}

/// Detailed share statistics.
#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleShareDetail {
    /// Total share users
    pub share_user: u32,
}

// ============== getbizsummary (Article Overview Summary) ==============

/// Overview summary of article performance for a given date.
#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleSummary {
    /// Date of statistics (YYYY-MM-DD)
    pub ref_date: String,
    /// Detailed overview metrics
    pub detail: ArticleSummaryDetail,
}

/// Detailed overview metrics summary.
#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleSummaryDetail {
    /// Total read users
    pub read_user: u32,
    /// Breakdown of readers by source
    pub read_user_source: Vec<ReadUserSource>,
    /// Total share users
    pub share_user: u32,
    /// Likes
    pub zaikan_user: u32,
    /// Thumbs up
    pub like_user: u32,
    /// Comment count
    pub comment_count: u32,
    /// Users who added to collections
    pub collection_user: u32,
    /// Origin redirect users
    pub redirect_ori_page_user: u32,
    /// Number of published articles
    pub send_page_count: u32,
}

// ============== getarticletotaldetail (Total Detail per article) ==============

/// Detailed statistics tracking a single article over time.
#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleTotalDetail {
    /// Date of publication (YYYY-MM-DD)
    pub ref_date: String,
    /// Message ID combined with the index, e.g. "12003_3"
    pub msgid: String,
    /// Publish Type
    pub publish_type: u32,
    /// Daily stat breakdowns since publication
    pub detail_list: Vec<ArticleStatDetail>,
}

/// Daily detailed statistics tracking for an article since publication.
#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleStatDetail {
    /// Stat date (YYYY-MM-DD)
    pub stat_date: String,
    pub read_user: u32,
    pub read_user_source: Vec<ReadUserSource>,
    pub share_user: u32,
    pub zaikan_user: u32,
    pub like_user: u32,
    pub comment_count: u32,
    pub collection_user: u32,
    #[serde(default)]
    pub praise_money: u32,
    #[serde(default)]
    pub read_subscribe_user: u32,
    #[serde(default)]
    pub read_delivery_rate: f64,
    #[serde(default)]
    pub read_finish_rate: f64,
    #[serde(default)]
    pub read_avg_activetime: f64,
    #[serde(default)]
    pub read_jump_position: Vec<ReadJumpPosition>,
}

/// User drop-off positions within the article text.
#[derive(Debug, Deserialize, Serialize)]
pub struct ReadJumpPosition {
    /// Position quartile: 1: 0-20%, 2: 20-40%, etc.
    pub position: u32,
    /// Rate of drop off at this position
    pub rate: f64,
}

/// Client for Datacube Statistics APIs.
#[derive(Debug, Clone)]
pub struct DatacubeClient {
    http_client: Arc<WeChatHttpClient>,
    token_manager: Arc<TokenManager>,
}

impl DatacubeClient {
    /// Creates a new DatacubeClient.
    pub fn new(http_client: Arc<WeChatHttpClient>, token_manager: Arc<TokenManager>) -> Self {
        Self {
            http_client,
            token_manager,
        }
    }

    /// Fetches the daily article reading statistics. (Max 1 day range)
    ///
    /// Endpoint: `/datacube/getarticleread`
    pub async fn get_article_read(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<DatacubeResponse<ArticleReadTotal>> {
        debug!(
            "Fetching article reading stats from {} to {}",
            begin_date, end_date
        );
        let req = DatacubeRequest {
            begin_date: begin_date.to_string(),
            end_date: end_date.to_string(),
        };

        let access_token = self.token_manager.get_access_token().await?;
        let res = self
            .http_client
            .post_json_with_token("/cgi-bin/datacube/getarticleread", &access_token, &req)
            .await?;

        let wx_res: WeChatResponse<DatacubeResponse<ArticleReadTotal>> = res.json().await?;
        wx_res.into_result()
    }

    /// Fetches the daily article sharing statistics. (Max 1 day range)
    ///
    /// Endpoint: `/datacube/getarticleshare`
    pub async fn get_article_share(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<DatacubeResponse<ArticleShareTotal>> {
        debug!(
            "Fetching article share stats from {} to {}",
            begin_date, end_date
        );
        let req = DatacubeRequest {
            begin_date: begin_date.to_string(),
            end_date: end_date.to_string(),
        };

        let access_token = self.token_manager.get_access_token().await?;
        let res = self
            .http_client
            .post_json_with_token("/cgi-bin/datacube/getarticleshare", &access_token, &req)
            .await?;

        let wx_res: WeChatResponse<DatacubeResponse<ArticleShareTotal>> = res.json().await?;
        wx_res.into_result()
    }

    /// Fetches the high-level business overview for articles. (Max 30 day range)
    ///
    /// Endpoint: `/datacube/getbizsummary`
    pub async fn get_biz_summary(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<DatacubeResponse<ArticleSummary>> {
        debug!(
            "Fetching article biz summary from {} to {}",
            begin_date, end_date
        );
        let req = DatacubeRequest {
            begin_date: begin_date.to_string(),
            end_date: end_date.to_string(),
        };

        let access_token = self.token_manager.get_access_token().await?;
        let res = self
            .http_client
            .post_json_with_token("/cgi-bin/datacube/getbizsummary", &access_token, &req)
            .await?;

        let wx_res: WeChatResponse<DatacubeResponse<ArticleSummary>> = res.json().await?;
        wx_res.into_result()
    }

    /// Fetches the detailed long-term performance data for individual articles published during this period. (Max 1 day range)
    ///
    /// Endpoint: `/datacube/getarticletotaldetail`
    pub async fn get_article_total_detail(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<DatacubeResponse<ArticleTotalDetail>> {
        debug!(
            "Fetching article total detail from {} to {}",
            begin_date, end_date
        );
        let req = DatacubeRequest {
            begin_date: begin_date.to_string(),
            end_date: end_date.to_string(),
        };

        let access_token = self.token_manager.get_access_token().await?;
        let res = self
            .http_client
            .post_json_with_token(
                "/cgi-bin/datacube/getarticletotaldetail",
                &access_token,
                &req,
            )
            .await?;

        let wx_res: WeChatResponse<DatacubeResponse<ArticleTotalDetail>> = res.json().await?;
        wx_res.into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_article_read() {
        let json_data = json!({
            "list": [
                {
                    "ref_date": "2025-11-01",
                    "msgid": "10000050_1",
                    "detail": {
                        "read_user": 4123,
                        "read_user_source": [
                            {
                                "user_count": 4123,
                                "scene_desc": "全部"
                            },
                            {
                                "user_count": 234,
                                "scene_desc": "公众号消息"
                            }
                        ]
                    }
                }
            ],
            "is_delay": false
        });

        let response: DatacubeResponse<ArticleReadTotal> =
            serde_json::from_value(json_data).unwrap();
        assert!(!response.is_delay);
        assert_eq!(response.list.len(), 1);

        let item = &response.list[0];
        assert_eq!(item.ref_date, "2025-11-01");
        assert_eq!(item.msgid, "10000050_1");
        assert_eq!(item.detail.read_user, 4123);
        assert_eq!(item.detail.read_user_source.len(), 2);
        assert_eq!(item.detail.read_user_source[0].scene_desc, "全部");
    }

    #[test]
    fn test_deserialize_article_share() {
        let json_data = json!({
            "list": [
                {
                    "ref_date": "2025-11-01",
                    "msgid": "2247490098_1",
                    "detail": {
                        "share_user": 366
                    }
                }
            ],
            "is_delay": false
        });

        let response: DatacubeResponse<ArticleShareTotal> =
            serde_json::from_value(json_data).unwrap();
        assert_eq!(response.list[0].detail.share_user, 366);
    }

    #[test]
    fn test_deserialize_biz_summary() {
        let json_data = json!({
            "list": [
                {
                    "ref_date": "2025-11-01",
                    "detail": {
                        "read_user": 4123,
                        "read_user_source": [
                             {
                                "user_count": 4123,
                                "scene_desc": "全部"
                            },
                            {
                                "user_count": 234,
                                "scene_desc": "公众号消息"
                            }
                        ],
                        "share_user": 366,
                        "zaikan_user": 191,
                        "like_user": 386,
                        "comment_count": 33,
                        "collection_user": 233,
                        "redirect_ori_page_user": 369,
                        "send_page_count": 512
                    }
                }
            ],
            "is_delay": false
        });

        let response: DatacubeResponse<ArticleSummary> = serde_json::from_value(json_data).unwrap();

        let detail = &response.list[0].detail;
        assert_eq!(detail.read_user, 4123);
        assert_eq!(detail.share_user, 366);
        assert_eq!(detail.send_page_count, 512);
    }

    #[test]
    fn test_deserialize_total_detail() {
        let json_data = json!({
            "list": [
                {
                    "ref_date": "2025-11-01",
                    "msgid": "2247490098_1",
                    "publish_type": 0,
                    "detail_list": [
                        {
                            "stat_date": "2025-11-01",
                            "read_user": 4123,
                            "read_user_source": [
                                {
                                    "user_count": 4123,
                                    "scene_desc": "全部"
                                },
                            ],
                            "share_user": 366,
                            "zaikan_user": 191,
                            "like_user": 386,
                            "comment_count": 33,
                            "collection_user": 233,
                            "praise_money": 361,
                            "read_subscribe_user": 327,
                            "read_delivery_rate": 0.0271002,
                            "read_finish_rate": 0.6304348,
                            "read_avg_activetime": 1.0588236,
                            "read_jump_position": [
                                {
                                    "position": 1,
                                    "rate": 0.5304147
                                },
                                {
                                    "position": 2,
                                    "rate": 0.1023412
                                }
                            ]
                        }
                    ]
                }
            ],
            "is_delay": false
        });

        let response: DatacubeResponse<ArticleTotalDetail> =
            serde_json::from_value(json_data).unwrap();

        let item = &response.list[0];
        assert_eq!(item.publish_type, 0);
        assert_eq!(item.detail_list.len(), 1);

        let stat = &item.detail_list[0];
        assert_eq!(stat.stat_date, "2025-11-01");
        assert_eq!(stat.read_delivery_rate, 0.0271002);
        assert_eq!(stat.read_jump_position.len(), 2);
        assert_eq!(stat.read_jump_position[1].position, 2);
        assert_eq!(stat.read_jump_position[1].rate, 0.1023412);
    }
}
