mod new_geschichtlich_setze;
mod schwirigkeit_liste;
pub mod setze;

pub use new_geschichtlich_setze::NewGeschichtlichSetze as NewGeschichtlichSetzeStruct;
pub use schwirigkeit_liste::bulk_insert as SchwirigkeitListeBulkInsert;
pub use schwirigkeit_liste::fetch_all_schwirigkeit_list as SchwirigkeitListeFetchAll;
pub use setze as SetzeRepo;
