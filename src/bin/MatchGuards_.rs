#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn print_tile(tile: Tile) {
    use Tile::*;
    match tile {
        Brick(brick @ BrickStyle::Gray | brick @ BrickStyle::Red) => {
            println!("The brick color is {:?}", brick)
        }
        Brick(other) => println!("{:?} brick", other),
        Dirt | Grass | Sand => println!("Ground tile"),
        Treasure(TreasureChest { amount, .. }) if amount >= 100 => println!("Losts of gold"),
        Water(pressure) if pressure.0 < 10 => println!("Water pressure level:{}", pressure.0),
        Water(pressure) if pressure.0 >= 10 => println!("High water pressure!"),
        
        _ => (),
    }
}

fn main() {
    let tile = Tile::Brick(BrickStyle::Red);
    print_tile(tile);
    let tile = Tile::Sand;
    print_tile(tile);
    let tile = Tile::Treasure(TreasureChest { content: TreasureItem::Gold, amount: 200 });
    print_tile(tile);
    let tile = Tile::Water(Pressure(9));
    print_tile(tile);
}
