// use anyhow::Result;
// use std::cell::{Cell, RefCell};
//
// pub struct Cavern<'a> {
//     pub octopi: Vec<Octopus<'a>>,
// }
//
// pub struct Octopus<'a> {
//     pub energy_level: i8,
//     pub flashing: bool,
//     pub neighbors: RefCell<Vec<&'a Octopus<'a>>>,
//     pub next: Cell<Option<&'a Octopus<'a>>>,
// }
// impl Octopus {
//     pub fn increase_energy(&mut self) -> Result<&mut Self> {
//         self.energy_level += 1;
//
//         if self.energy_level >= 9 && !self.flashing {
//             self.flash();
//         }
//
//         Ok(self)
//     }
//
//     pub fn flash(&self) -> Result<()> {
//         for x in self.neighbors.borrow_mut().push {}
//
//         Ok(())
//     }
// }
// impl<'a> Iterator for Octopus<'a> {
//     type Item = Cell<&'a Self>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         *self.next
//     }
// }
//
// pub fn part_one(node: Octopus) -> Result<i64> {
//     node.into_iter().map(|octopus_cell| {
//         let octopus = octopus_cell.get().increase_energy()?;
//
//         octopus_cell.set(octopus);
//     });
//     //Step 1: Increase Energy
//     //Step 2: Cascade
//     //Step 3: Reset Energy Levels
//     Ok(54)
// }
