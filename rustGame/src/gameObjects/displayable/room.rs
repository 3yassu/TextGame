mod inventory;
pub struct Wall{
    wallInv: Inventory
}
pub struct RoomData{
    floorInv: Inventory,
    wallN: Wall,
    wallS: Wall,
    wallE: Wall,
    wallW: Wall,
    wallPntr: &Wall,
}
pub struct Room<'a, >{
    roomData: RoomData, 
    roomN: &'a Room,
    roomS: &'a Room,
    roomE: &'a Room,
    roomW: &'a Room
}