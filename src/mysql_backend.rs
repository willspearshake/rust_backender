pub mod mysql_operations {

    use mysql::*;
    use mysql::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
    pub struct Person {
        pub person_id: i32,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub address: Option<String>,
        pub city: Option<String>,
    }

    pub fn insert_people(people: Vec<Person>) -> Result<(), Error> {

        //Connecting and creating pool
        let url = "mysql://the_cat:password@localhost:3306/the_fun_db";

        let pool = Pool::new(url)?;

        let mut conn = pool.get_conn()?;
        

        // Now let's insert persons to the database
        conn.exec_batch(
            r"INSERT INTO People (PersonId,  FirstName, LastName, Address, City)
            VALUES (:person_id,  :first_name, :last_name, :address, :city)",
            people.iter().map(|p| params! {
                "person_id" => p.person_id,
                "first_name" => &p.first_name,
                "last_name" => &p.last_name,
                "address" => &p.address,
                "city" => &p.city,
            })
        )?;

        Ok(())
    }

    pub fn select_all_people() -> Result<Vec<Person>, Error> {

        //Connecting and creating pool
        let url = "mysql://the_cat:password@localhost:3306/the_fun_db";

        let pool = Pool::new(url)?;

        let mut conn = pool.get_conn()?;
        
        let selected_persons = 
        conn.query_map(
            "SELECT * from People",
            |(person_id,  first_name,last_name, address, city)| {
                Person { person_id,  first_name,last_name, address, city }
            },
        )?;

        Ok(selected_persons)
    }



}



