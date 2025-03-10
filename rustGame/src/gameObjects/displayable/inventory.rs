mod display;
pub struct Items{
    itemName: String, //Item Name, Name of the Item
    itemID: i32, //Item ID, Numerical ID for items
    itemType: i32, //Item Type, What it can do
    itemDura: i32, //Item Durability, How long it lasts {0=forever, 1=unusable}
}
impl Items{
    //Public methods
    pub fn new(itemName: String, itemID: i32, itemType: i32, itemDura: i32) -> Self{
        Self{itemName, itemID, itemType, itemDura}
    }
}
impl Printable for Items{
    fn print(&self){
        println!("Item Name: {}", self.itemName);
        println!("Item ID: {}", self.itemID);
        println!("Item Type: {}", self.itemType);
        println!("Item Durability: {} (0 = Unbreakable, 1 = Final Use)", self.itemDura);
    }
}
//Inventory Class Definition
pub struct Inventory{
    invList: Vec<Items>,
    maxLength: i32
}
impl Inventory{
    //Public Methods
    pub fn new(maxLength: i32) -> Self{
        Self{invList: Vec::new(), maxLength}
    }
    pub fn addItem(&mut self, item: Items){
        if(self.maxLength > self.invList.len().try_into().unwrap()){
            println!("- {} Added To Inventory - ", item.itemName);
            self.invList.push(item);
        }else{
            println!("- Inventory Full! -")
        }
    }
}
impl Printable for Inventory{
    fn print(&mut self){
        for name in &self.invList {
            print!("-{}", name.itemName);
        }   println!("-");
    }  
}


