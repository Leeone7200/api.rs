/*
 * @solvedac/unofficial-documentation
 *
 * 이 프로젝트는 [solved.ac](https://solved.ac/) API를 문서화하는 커뮤니티 프로젝트입니다. 이 저장소는 원작자의 요청에 따라 언제든 지워질 수 있으며, 현재 API와 일치하지 않을 수도 있는 점 양해 부탁드립니다.  <sup>   solved.ac 서비스는 shiftpsh가 기획·개발·디자인·운영하는 프로젝트로,   이 저장소와는 solved.ac의 API를 문서화해둔 것 이외에는 아무런 관련이 없습니다. </sup>  [GitHub에서 보기](https://github.com/solvedac/unofficial-documentation)  ![@solvedac/unofficial-documentation banner](./assets/solvedac-ud-compact.png) 
 *
 * The version of the OpenAPI document: 3.2021.09+b2
 * Contact: public.ranolp@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// User : 사용자 정보입니다. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// 사용자명입니다.
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// 사용자의 자기소개입니다.
    #[serde(rename = "bio", skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// 사용자가 속한 조직 목록입니다.
    #[serde(rename = "organizations", skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<crate::models::Schema11>>,
    #[serde(rename = "badge", skip_serializing_if = "Option::is_none")]
    pub badge: Option<Box<crate::models::UserBadge>>,
    #[serde(rename = "background", skip_serializing_if = "Option::is_none")]
    pub background: Option<Box<crate::models::UserBackground>>,
    /// 사용자의 프로필 사진으로 가는 하이퍼링크입니다.
    #[serde(rename = "profileImageUrl", skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<String>,
    /// 사용자가 푼 문제 수입니다.
    #[serde(rename = "solvedCount", skip_serializing_if = "Option::is_none")]
    pub solved_count: Option<i64>,
    /// 사용자가 난이도 기여를 한 횟수입니다.
    #[serde(rename = "voteCount", skip_serializing_if = "Option::is_none")]
    pub vote_count: Option<i64>,
    /// 사용자가 여태까지 획득한 경험치량입니다.
    #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    /// Bronze V를 1, Bronze IV를 2, ..., Ruby I을 30, Master를 31로 표현하는 사용자 티어입니다. 자세한 값 정보는 표1. 수치 - 이름 표를 펼쳐 참고하십시오.  <details>   <summary>     표1. 수치 - 이름 표   </summary>    | 수치 | 이름         |   | ---: | ------------ |   |    1 | Bronze V     |   |    2 | Bronze IV    |   |    3 | Bronze III   |   |    4 | Bronze II    |   |    5 | Bronze I     |   |    6 | Silver V     |   |    7 | Silver IV    |   |    8 | Silver III   |   |    9 | Silver II    |   |   10 | Silver I     |   |   11 | Gold V       |   |   12 | Gold IV      |   |   13 | Gold III     |   |   14 | Gold II      |   |   15 | Gold I       |   |   16 | Platinum V   |   |   17 | Platinum IV  |   |   18 | Platinum III |   |   19 | Platinum II  |   |   20 | Platinum I   |   |   21 | Diamond V    |   |   22 | Diamond IV   |   |   23 | Diamond III  |   |   24 | Diamond II   |   |   25 | Diamond I    |   |   26 | Ruby V       |   |   27 | Ruby IV      |   |   28 | Ruby III     |   |   29 | Ruby II      |   |   30 | Ruby I       |   |   31 | Master       |  </details> 
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<i64>,
    /// 사용자의 레이팅입니다.
    #[serde(rename = "rating", skip_serializing_if = "Option::is_none")]
    pub rating: Option<i64>,
    /// 푼 문제의 난이도 합으로 계산한 사용자의 레이팅입니다.
    #[serde(rename = "ratingByProblemsSum", skip_serializing_if = "Option::is_none")]
    pub rating_by_problems_sum: Option<i64>,
    /// 취득한 클래스에 따른 사용자의 레이팅입니다.
    #[serde(rename = "ratingByClass", skip_serializing_if = "Option::is_none")]
    pub rating_by_class: Option<i64>,
    /// 푼 문제 수로 계산한 사용자의 레이팅입니다.
    #[serde(rename = "ratingBySolvedCount", skip_serializing_if = "Option::is_none")]
    pub rating_by_solved_count: Option<i64>,
    /// 문제 난이도에 기여한 횟수로 계산한 사용자의 레이팅입니다.
    #[serde(rename = "ratingByVoteCount", skip_serializing_if = "Option::is_none")]
    pub rating_by_vote_count: Option<i64>,
    /// 사용자가 취득한 Class입니다.
    #[serde(rename = "class", skip_serializing_if = "Option::is_none")]
    pub class: Option<i64>,
    /// 사용자가 취득한 Class의 수준입니다.
    #[serde(rename = "classDecoration", skip_serializing_if = "Option::is_none")]
    pub class_decoration: Option<ClassDecoration>,
    /// 사용자의 라이벌 수입니다.
    #[serde(rename = "rivalCount", skip_serializing_if = "Option::is_none")]
    pub rival_count: Option<i64>,
    /// 사용자의 역라이벌 수입니다.
    #[serde(rename = "reverseRivalCount", skip_serializing_if = "Option::is_none")]
    pub reverse_rival_count: Option<i64>,
    /// 최대 연속 문제 풀이일 수입니다.
    #[serde(rename = "maxStreak", skip_serializing_if = "Option::is_none")]
    pub max_streak: Option<i64>,
}

impl User {
    /// 사용자 정보입니다. 
    pub fn new() -> User {
        User {
            handle: None,
            bio: None,
            organizations: None,
            badge: None,
            background: None,
            profile_image_url: None,
            solved_count: None,
            vote_count: None,
            exp: None,
            tier: None,
            rating: None,
            rating_by_problems_sum: None,
            rating_by_class: None,
            rating_by_solved_count: None,
            rating_by_vote_count: None,
            class: None,
            class_decoration: None,
            rival_count: None,
            reverse_rival_count: None,
            max_streak: None,
        }
    }
}

/// 사용자가 취득한 Class의 수준입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClassDecoration {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "silver")]
    Silver,
    #[serde(rename = "gold")]
    Gold,
}

