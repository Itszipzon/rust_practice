use entity::player::Player;
use item::equipment::Equipment;
use items::{list::ItemList, sword::Sword};

mod item;
mod items;
mod entity;

fn main() {
    println!("========= Generating Player =========");
    let mut player = Player::new(
        "Hero".to_string(),
        "A brave hero".to_string(),
        10,
        1,
        100,
        50,
    );

    println!("Player: {:?}", player);

    println!("========= Generating Sword =========");
    let mut iron_sword = Sword::new(
        "A sharp iron sword".to_string(),
        15,
        5,
        145,
        ItemList::IronSword
    );

    println!("Sword: {:?}", iron_sword);

    iron_sword.as_item_mut().unwrap().set_name("God Sword!".to_string());

    println!("========= Adding Sword to Player Inventory =========");
    player.add_item(Box::new(iron_sword));
    println!("Player: {:?}", player);

    println!("========= Spawning Zombie =========");
    let mut zombie = entity::zombie::Zombie::new(
        1,
        "Zombie".to_string(),
        "A scary zombie".to_string(),
        65,
        5,
    );
    println!("Zombie: {:?}", zombie);

    println!("========= Start fight between Player and Zombie =========");
    println!("{:?}", player);
    println!("{:?}", zombie);
    println!("========= Fight =========");
    while player.is_alive() && zombie.is_alive() {
        println!("Player attacks Zombie!");
        let damage = player.attack();
        zombie.take_damage(damage);
        println!("Zombie takes {} damage!", damage);
        println!("Zombie health: {}", zombie.get_health());

        println!("Weapon durability: {}", player.inventory[player.active_item as usize].as_ref().unwrap().as_equipment().unwrap().as_weapon().unwrap().durability());

        if !zombie.is_alive() {
            println!("Zombie is dead!");
            break;
        }

        println!("Zombie attacks Player!");
        let damage = zombie.attack();
        player.take_damage(damage);
        println!("Player takes {} damage!", damage);
        println!("Player health: {}", player.get_health());

        if !player.is_alive() {
            println!("Player is dead!");
            break;
        }
    }

    println!("========= End fight =========");
    println!("{:?}", zombie);
    println!("{:?}", player);
    println!("========= End of Game =========");
}
