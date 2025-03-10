pub struct UserData{
    endGame: bool,
    userInfo: String,

}
pub struct BinaryNode<'a, T>{
    entry: UserData,
    left: Option<&'a BinaryNode>,
    right: Option<&'a BinaryNode>
}