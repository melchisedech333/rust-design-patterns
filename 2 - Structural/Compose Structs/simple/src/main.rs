
/*
Este código contem um problema com empréstimos. Por mais que seja possível
resolver esses problemas sem modificar muito a estrutura do código, ele ainda
mantem as coisas muito acopladas na struct Database. O ideal é decompor a
estrutura de dados em tipos personalizados, de modo a obter assim maior controle
das coisas.

    struct Database {
        connection_string: String,
        timeout: u32,
        pool_size: u32,
    }

    fn print_database(database: &Database) {
        println!("Connection string: {}", database.connection_string);
        println!("Timeout: {}", database.timeout);
        println!("Pool size: {}", database.pool_size);
    }

    fn main() {
        let mut db = Database {
            connection_string: "initial string".to_string(),
            timeout: 30,
            pool_size: 100,
        };

        let connection_string = &mut db.connection_string;
        print_database(&db);  // Immutable borrow of `db` happens here
        // *connection_string = "new string".to_string();  // Mutable borrow is used
                                                        // here
    }


Segue abaixo uma possível maneira de se resolver o problema.

    #[derive(Clone)]
    struct Database {
        connection_string: String,
        timeout: u32,
        pool_size: u32,
    }

    fn print_database2(database: &Database) {
        println!("Connection string: {}", database.connection_string);
        println!("Timeout: {}", database.timeout);
        println!("Pool size: {}", database.pool_size);
    }

    fn print_database1(connection_string: String, timeout: u32, pool_size: u32) {
        println!("Connection string: {}", connection_string);
        println!("Timeout: {}", timeout);
        println!("Pool size: {}", pool_size);
    }

    fn main() {
        let mut db = Database {
            connection_string: "initial string".to_string(),
            timeout: 30,
            pool_size: 100,
        };

        let connection_string = &mut db.connection_string;
        print_database1(connection_string.clone(), 1, 2);  
        *connection_string = "new string".to_string();
        print_database1(connection_string.clone(), 1, 2);
        print_database2(&db);
    }
*/

/// Segue abaixo a maneira mais elegante de se resolver o problema.

// Database is now composed of three structs - ConnectionString, Timeout and PoolSize.
// Let's decompose it into smaller structs
#[derive(Debug, Clone)]
struct ConnectionString(String);

#[derive(Debug, Clone, Copy)]
struct Timeout(u32);

#[derive(Debug, Clone, Copy)]
struct PoolSize(u32);

// We then compose these smaller structs back into `Database`
struct Database {
    connection_string: ConnectionString,
    timeout: Timeout,
    pool_size: PoolSize,
}

// print_database can then take ConnectionString, Timeout and Poolsize struct instead
fn print_database(connection_str: ConnectionString, 
                  timeout: Timeout, 
                  pool_size: PoolSize) {
    println!("Connection string: {:?}", connection_str);
    println!("Timeout: {:?}", timeout);
    println!("Pool size: {:?}", pool_size);
}

fn main() {
    // Initialize the Database with the three structs
    let mut db = Database {
        connection_string: ConnectionString("localhost".to_string()),
        timeout: Timeout(30),
        pool_size: PoolSize(100),
    };

    let connection_string = &mut db.connection_string;
    print_database(connection_string.clone(), db.timeout, db.pool_size);
    *connection_string = ConnectionString("new string".to_string());
}


