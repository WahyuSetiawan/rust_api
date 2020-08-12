mod connection;

pub fn testing_insert_data(){
    let conection = connection::connection();

    if let Ok(_) = conection{
        
    }else{
        println!("no success connection");
    }
}