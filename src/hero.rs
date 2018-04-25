extern crate rusqlite;
use self::rusqlite::Connection;

#[derive(Serialize, Deserialize, Debug)]
pub struct Hero {
    pub id:Option<i32>,
    pub name:String,
    pub identity:String,
    pub hometown:String,
    pub age:i32,
}

impl Hero {
    pub fn create(conn: &Connection, hero: Hero) -> Hero {
        conn.execute(
            "insert into heroes(name, identity, hometown, age) values(?1, ?2, ?3, ?4)",
            &[&hero.name, &hero.identity, &hero.hometown, &hero.age]
        ).expect("insert hero");

        let hero: Hero = conn.query_row(
            "select * from heroes order by id desc limit 1",
            &[], |row| {
                Hero {
                    id: row.get(0),
                    name: row.get(1),
                    identity: row.get(2),
                    hometown: row.get(3),
                    age: row.get(4)
                }
            }
        ).unwrap();

        hero
    }

    pub fn read(conn: &Connection) -> Vec<Hero> {
        let mut stmt = conn.prepare("select * from heroes").unwrap();
        let hero_iter = stmt.query_map(&[], |row| {
            Hero {
                id: row.get(0),
                name: row.get(1),
                identity: row.get(2),
                hometown: row.get(3),
                age: row.get(4)
            }
        }).unwrap();
        let mut heroes:Vec<Hero> = Vec::new();
        for hero in hero_iter {
            heroes.push(hero.unwrap());
        }

        heroes
    }

    pub fn read_single(conn: &Connection, id: i32) -> Hero {
        let mut stmt = conn.prepare("select * from heroes where id=?1").unwrap();
        let hero = stmt.query_row(&[&id], |row| {
            Hero {
                id: row.get(0),
                name: row.get(1),
                identity: row.get(2),
                hometown: row.get(3),
                age: row.get(4)
            }
        }).unwrap();

        hero
    }

    pub fn update(conn: &Connection, id: i32, hero: &Hero) -> Hero {
        conn.execute(
            "update heroes set name=?2, identity=?3, hometown=?4, age=?5 where id=?1",
            &[&id, &hero.name, &hero.identity, &hero.hometown, &hero.age]
        ).expect("update hero");

        let hero: Hero = conn.query_row(
            "select * from heroes where id=?1 limit 1",
            &[&id], |row| {
                Hero {
                    id: row.get(0),
                    name: row.get(1),
                    identity: row.get(2),
                    hometown: row.get(3),
                    age: row.get(4)
                }
            }
        ).unwrap();

        hero
    }

    pub fn delete(conn: &Connection, id: i32) -> bool {
        conn.execute(
            "delete from heroes where id=?1",
            &[&id]
        ).is_ok()
    }
}