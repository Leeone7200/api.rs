/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다. 이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.  <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>  [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)  **주의**: (2023/03/08~) CORS 문제로 인해 API는 사이트 내에서 호출할 수 없으므로 별도 도구를 이용해주십시오. ([#51](https://github.com/solvedac/unofficial-documentation/issues/51))  ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png) 
 *
 * The version of the OpenAPI document: 3.2022.02+b1
 * Contact: public.ranolp@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// Badge : 사용자가 사용할 수 있는 뱃지입니다. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Badge {
    /// 뱃지의 ID입니다.
    #[serde(rename = "badgeId")]
    pub badge_id: String,
    /// 뱃지 사진으로 가는 하이퍼링크입니다.
    #[serde(rename = "badgeImageUrl")]
    pub badge_image_url: String,
    /// 뱃지의 이름입니다.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 뱃지의 설명입니다.
    #[serde(rename = "displayDescription")]
    pub display_description: String,
    #[serde(rename = "badgeTier", skip_serializing_if = "Option::is_none")]
    pub badge_tier: Option<crate::models::BadgeTier>,
    #[serde(rename = "badgeCategory", skip_serializing_if = "Option::is_none")]
    pub badge_category: Option<crate::models::BadgeCategory>,
}

impl Badge {
    /// 사용자가 사용할 수 있는 뱃지입니다. 
    pub fn new(badge_id: String, badge_image_url: String, display_name: String, display_description: String) -> Badge {
        Badge {
            badge_id,
            badge_image_url,
            display_name,
            display_description,
            badge_tier: None,
            badge_category: None,
        }
    }
}


