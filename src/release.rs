#[allow(dead_code)]
pub struct ReleaseOptions {
    pub token: String,
    pub repo: String,
    pub tag: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
}
