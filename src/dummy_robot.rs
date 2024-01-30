use std::thread::sleep;
use std::time;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{Direction, go, robot_map, robot_view};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;

pub struct DummyRobot(Robot);

impl Default for DummyRobot {
    fn default() -> Self {
        DummyRobot(Robot::default())
    }
}

impl Runnable for DummyRobot {
    fn process_tick(&mut self, world: &mut World) {
        //sleep(time::Duration::from_secs(1));

        println!("VIEW AROUND");
        let robot_view= robot_view(self, world);
        for row in robot_view.iter() {
            for col in row.iter() {
                match col {
                    None => println!("default_unknown_tile"),
                    Some(tile) => println!("{:?}", tile.tile_type),
                }
            }
        }

        // println!("GET MAP");
        // let robot_world = robot_map(world).unwrap();
        // for row in robot_world.iter() {
        //     for elem in row.iter() {
        //         match elem {
        //             None => println!("No tile"),
        //             Some(tile) => println!("{:?}", tile),
        //         }
        //     }
        // }

        //go(self, world, Direction::Left).expect("Error in go function");
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Ready => {}
            Event::Terminated => {}
            Event::TimeChanged(_) => {}
            Event::DayChanged(_) => {}
            Event::EnergyRecharged(_) => {}
            Event::EnergyConsumed(_) => {}
            Event::Moved(_, _) => {}
            Event::TileContentUpdated(_, _) => {}
            Event::AddedToBackpack(_, _) => {}
            Event::RemovedFromBackpack(_, _) => {}
        }
    }

    fn get_energy(&self) -> &Energy {
        &self.0.energy
    }

    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.0.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.0.coordinate
    }

    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.0.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.0.backpack
    }

    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.0.backpack
    }
}