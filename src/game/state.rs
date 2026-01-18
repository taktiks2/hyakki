#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameState {
    MainMenu,
    #[default]
    Playing,
    ShowInventory,
    SelectSpell,
    GameOver,
}
