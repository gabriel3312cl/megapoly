export type SpaceData = {
    id: number;
    name: string;
    type: 'Property' | 'Railroad' | 'Utility' | 'Action' | 'Tax' | 'Corner';
    color?: string;
    price?: number;
};

export const boardData: SpaceData[] = [
    // Bottom Row (Right to Left) - Index 0 is GO
    { id: 0, name: "GO", type: "Corner" }, // Bottom Right
    { id: 1, name: "Mediterranean Ave", type: "Property", color: "#955438", price: 60 },
    { id: 2, name: "Community Chest", type: "Action" },
    { id: 3, name: "Baltic Ave", type: "Property", color: "#955438", price: 60 },
    { id: 4, name: "Income Tax", type: "Tax", price: 200 },
    { id: 5, name: "Reading RR", type: "Railroad", price: 200 },
    { id: 6, name: "Oriental Ave", type: "Property", color: "#aae0fa", price: 100 },
    { id: 7, name: "Chance", type: "Action" },
    { id: 8, name: "Vermont Ave", type: "Property", color: "#aae0fa", price: 100 },
    { id: 9, name: "Connecticut Ave", type: "Property", color: "#aae0fa", price: 120 },

    // Left Column (Bottom to Top)
    { id: 10, name: "Jail / Visit", type: "Corner" }, // Bottom Left
    { id: 11, name: "St. Charles Place", type: "Property", color: "#d93a96", price: 140 },
    { id: 12, name: "Electric Company", type: "Utility", price: 150 },
    { id: 13, name: "States Ave", type: "Property", color: "#d93a96", price: 140 },
    { id: 14, name: "Virginia Ave", type: "Property", color: "#d93a96", price: 160 },
    { id: 15, name: "Pennsylvania RR", type: "Railroad", price: 200 },
    { id: 16, name: "St. James Place", type: "Property", color: "#f7941d", price: 180 },
    { id: 17, name: "Community Chest", type: "Action" },
    { id: 18, name: "Tennessee Ave", type: "Property", color: "#f7941d", price: 180 },
    { id: 19, name: "New York Ave", type: "Property", color: "#f7941d", price: 200 },

    // Top Row (Left to Right)
    { id: 20, name: "Free Parking", type: "Corner" }, // Top Left
    { id: 21, name: "Kentucky Ave", type: "Property", color: "#ed1b24", price: 220 },
    { id: 22, name: "Chance", type: "Action" },
    { id: 23, name: "Indiana Ave", type: "Property", color: "#ed1b24", price: 220 },
    { id: 24, name: "Illinois Ave", type: "Property", color: "#ed1b24", price: 240 },
    { id: 25, name: "B. & O. RR", type: "Railroad", price: 200 },
    { id: 26, name: "Atlantic Ave", type: "Property", color: "#fef200", price: 260 },
    { id: 27, name: "Ventnor Ave", type: "Property", color: "#fef200", price: 260 },
    { id: 28, name: "Water Works", type: "Utility", price: 150 },
    { id: 29, name: "Marvin Gardens", type: "Property", color: "#fef200", price: 280 },

    // Right Column (Top to Bottom)
    { id: 30, name: "Go To Jail", type: "Corner" }, // Top Right
    { id: 31, name: "Pacific Ave", type: "Property", color: "#1fb25a", price: 300 },
    { id: 32, name: "North Carolina Ave", type: "Property", color: "#1fb25a", price: 300 },
    { id: 33, name: "Community Chest", type: "Action" },
    { id: 34, name: "Pennsylvania Ave", type: "Property", color: "#1fb25a", price: 320 },
    { id: 35, name: "Short Line", type: "Railroad", price: 200 },
    { id: 36, name: "Chance", type: "Action" },
    { id: 37, name: "Park Place", type: "Property", color: "#0072bb", price: 350 },
    { id: 38, name: "Luxury Tax", type: "Tax", price: 100 },
    { id: 39, name: "Boardwalk", type: "Property", color: "#0072bb", price: 400 },
];
