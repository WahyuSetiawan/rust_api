mod connection;

use crate::model;
pub use connection::connection;

// pub fn testing_insert_data(){
//     let conection = connection::connection();

//     if let Ok(connection_success) = conection{
//         if let Ok(status) =  insert::insert_people(connection_success, model::Employee{
//             people_id: 1,
//             name: "wahyu setiawan".to_string(),
//         }){
//             print!("status insert success {:?}", status);
//         }else{
//             println!("status not success insert data {}","wahyu setiawan".to_string() );
//         }
//     }else{
//         println!("no success connection");
//     }
// }