package main;

import "net"
import "log"


func main() {
    listener,err:=net.Listen("tcp6", ":1234")
    if err!=nil{
        log.Fatalln("Error (net.Listen):", err)
    }
    defer listener.Close()
    log.Println("Listening on:", listener.Addr())

    for{
        connection, err:=listener.Accept()
        if err != nil {
            log.Println("Error (listener.Accept):", err)
        }

        go func(connection net.Conn){
            defer connection.Close()
            buffer:=make([]byte, 1000)
            n,err:=connection.Read(buffer)
            if err!=nil{
                log.Println("Error (connection.Read):", err)
                return
            }

            log.Printf("%s->%s: [%d] %s", connection.RemoteAddr(), connection.LocalAddr(), n, string(buffer[:n]))
        }(connection)
    }
}