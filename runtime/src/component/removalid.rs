#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum RemovalId {
    Scene,
    GameplayUi,
    MenuUi,
    PauseUi,
    ResultUi,
    MapSelectUi,
    LoginUi,
}
