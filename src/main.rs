use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    net::TcpListener, sync::broadcast,
};
// This is the type that will be used to listen for incoming connections
// AsyncReadExt is a trait that will be used to read data from the socket
// AsyncWriteExt is a trait that will be used to write data to the socket

#[tokio::main] // This is the macro that will be used to create the runtime
async fn main() {
    // listen for incoming connections, as tcpListener is a future, we need to await it as it will suspends the
    // execution of the current thread until the future is resolved and returns the result which is a TcpListener or
    // an error
    let listener = TcpListener::bind("localhost:8080").await.unwrap();


    // to make the server actually chat server we need to communicate the line thats read 
    // from one client out to every single client that is connected at the moment to the server
    // for that we will use tokio broadcast channel, it is a channel that can be used to broadcast messages to multiple
    // receivers, it is a multi-producer, multiple-consumer channel, it means that we can have multiple producers that can
    // send messages to the channel and we can have a multiple consumer that can receive messages from the channel


    let (tx, rx) = broadcast::channel(10); // we will use the broadcast method to create a new channel, 
    //it will return a tuple of sender and receiver, it is taking 10 as the capacity of the channel, it means that we can 
    // store 10 messages in the channel, if we try to send more than 10 messages, the oldest message will be removed from
    // the channel and the new message will be added to the channel


    // outer loop for accepting multiple connections, now we can accept multiple connections and open multiple sockets
    // inner loop for reading and writing data from the socket continuously
    loop {
        // Now we need to accept incoming connections, accept methood will be called on the TcpLSistener and it will return
        // a future that will resolve to a tuple of TcpStream and SocketAddr, TcpStream is basically a connection and
        //socketAddr is the address of the connection.
        // we are accepting a single connection
        let (mut socket, addr) = listener.accept().await.unwrap();


        let tx = tx.clone(); 
        //we will use the cloned tx variable, it is becuase we are using the same tx variable in the outer loop, if we use
        // the same tx variable, it will give an error, so we will use the cloned tx variable
        // clone method is basically used to clone the variable and we hafve to use it because we are using the same tx and
        // value was moved into the outer loop, so we will use the clone method to clone the tx variable

        let mut rx = tx.subscribe();  
        // it will return a new receiver because subscribe method is used to create a new receiver, we will use the cloned 
        // tx variable to create a new receiver and make it mutable because we will use it in the inner loop

        // tokio spawn will help us to handle multiple connections concurrently,
        // we will use the tokio::spawn method to spawn a new task, this will return a future that will resolve to a result
        tokio::spawn(async move { // tokio spawn is used to spawn a new task which will run 
            //concurrently with the current task, async block is used to create a future, async move is used to move the socket
            // async block acts same as an async function, but difference is that async block can be used inside a function
            // and it takes all the refernces variables from the outer scope and moves them into the async block


        // To handle the borrow checker, we will split the socket into reader and writer, so that
        // we can use the reader and writer independently
        let (reader, mut writer) = socket.split(); // split the socket into reader and writer

        // we need to read something from the client, want to read some new data into memory from a network stream
        // which is also called socket, we need to create a buffer to store the data, we will use a vector of u8
        // stack allocated array of 1024 bytes,
        // buffer is used to store the data that is read from the socket
        //let mut buffer = [0u8; 1024]; // 1024 bytes

        // we will use the buffRead method on the socket to read the data from the socket
        // bufRead is better than buffer variable because it do all the work for us

        // it wraps any kind of reader and maintains its own buffer. and higher level operations can be performed on it
        // like read an entire line or read a single word from a stream
        let mut reader = tokio::io::BufReader::new(reader);

        let mut line = String::new();

        // using loop to continuously read and write data from the socket
        loop {

            // select macro is basically used to select between multiple futures, it will wait for any of the futures to
            // complete and return the result, it will return the result of the future that is completed first
            tokio::select! {
                // here we are using the read_line method to read a line from the socket and store it in the line variable
                // if result is Ok and the number of bytes read is 0, it means that the client has closed the connection
                // so we will break the loop
                result = reader.read_line(&mut line) => {
                    if result.unwrap() == 0 {
                        break;
                    }
                    // we will use the broadcast method to send the data to all the receivers
                    // line.clone() is used to clone the line variable, because we are using the same line
                    // variable in the outer loop
                    // than we are clearing the line variable
                    // so it won't send the same message again and again, duplicate messages will be sent if we don't clear
                    // the line variable,

                    // we are also sending the address of the client, so that we can differentiate between the clients
                    tx.send((line.clone(), addr)).unwrap();
                    line.clear();
                }
                // here result is the message that is received from the channel
                // we will use the write_all method to write the message to the socket,
                // msg and other_addr are the message and address that is received from the channel
                // other addr is the address of the client that sent the message
                // if the address of the client that sent the message is equal to the address of the client that is
                // connected to the server, we will write the message to the socket
                result = rx.recv() => {
                    let (msg, other_addr) = result.unwrap(); 
                    if addr == other_addr { // 
                        writer.write_all(msg.as_bytes()).await.unwrap();
                    }
                }
            }
/* 

            // bytes_read is the number of bytes that are read from the socket
            //let bytes_read = socket.read(&mut buffer).await.unwrap(); // read the data from the socket and store it in the buffer

            // read_line will read a line from the socket and store it in the line variable
            let bytes_read = reader.read_line(&mut line).await.unwrap();

            // if the number of bytes read is 0, it means that the client has closed the connection
            if bytes_read == 0 {
                break;
            }

            // we will use the broadcast method to send the data to all the receivers
            // we will use the send method to send the data to the channel, it will return a result
            // if the result is Ok, it means that the data is sent to all the receivers
            // if the result is Err, it means that the data is not sent to all the receivers
            // we will unwrap the result to get the value
            tx.send(line.clone()).unwrap(); // we will use the clone method to clone the line variable



            // rx.recv is the function that will receive the data from the channel, 
            // we are putting it in msg variable
            // and will put it in write_all method to write the data to the socket
            let msg = rx.recv().await.unwrap(); 



            // send the data back to the client, we will use the write method on the socket
            // write_all will write the data to the socket and will return a future that will resolve to a result
            // buffer[..bytes_read] means that we will write the data from the buffer starting from the
            //first byte to the last byte
            //socket.write_all(&buffer[..bytes_read]).await.unwrap();

            // line.as_bytes() will convert the line into bytes and write it to the socket
            // we will use the write_all method to write the data to the socket
            writer.write_all(msg.as_bytes()).await.unwrap();

            // clear the line variable
            line.clear(); // if not : 1st line will be printed twice, */
        }
    });
    }
}

// if the broadcast channel that tokio provides is not enough, we can use the mpsc channel that is provided by tokio.

// tokio::select! used when you have same shared state between finite number of things
// e.g: we have exactly two async task that need to run concurrently and they both need to access the same shared state
// 1 was reading the line and other was recieving the message from the channel, we dont have the third thing
// and socket split have two halves, one is reader and other is writer, but if we call tokiospawn on read line and rx.recv 
// we will be trying to split the reference on multiple tasks and it will not work because spawn needs static lifetime
// and we can't have static lifetime on the reference, so we will use tokio::select! macro


// turbofish operator is used to specify the type of the variable, it is used to specify the type of the variable
// when the compiler is not able to infer the type of the variable

// e.g: lets say I want to write a function that returns the default value of any type,
// but if I don't specify the type of the variable, the compiler will not be able to infer the type of the variable
// and turbofish operator will be used to specify the type of the variable
// fn default_value<T>() -> T {
//     T::default()
// }
// default_value::<i32>(); // turbofish operator is used to specify the type of the variable
// default_value::<String>(); // turbofish operator is used to specify the type of the variable
// here <> is the turbofish operator
