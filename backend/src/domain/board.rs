// Board logic
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub spaces: Vec<Space>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    pub id: usize,
    pub name: String,
    pub kind: SpaceKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "details")]
pub enum SpaceKind {
    Property(PropertyDetails),
    Railroad(VehicleDetails),
    Utility(UtilityDetails),
    Tax { amount: i64 },
    Action(ActionType), // Chance, Community Chest
    Go { reward: i64 },
    Jail,
    GoToJail,
    FreeParking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDetails {
    pub price: i64,
    pub rent: Vec<i64>, // Base, 1 house, ..., hotel
    pub house_cost: i64,
    pub color: ColorGroup,
    pub mortgage_value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleDetails {
    pub price: i64,
    pub mortgage_value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilityDetails {
    pub price: i64,
    pub mortgage_value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorGroup {
    Brown,
    LightBlue,
    Pink,
    Orange,
    Red,
    Yellow,
    Green,
    DarkBlue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Chance,
    CommunityChest,
}
