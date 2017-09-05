package main;

import "net"
import "log"


func main() {
    connection,err:=net.Dial("udp6", "[::1]:1234")
    if err!=nil{
        log.Fatalln("Error (net.Dial):", err)
    }
    defer connection.Close()

    n,err:=connection.Write([]byte("Hello(udp) from (go)client"))
    if err!=nil{
        log.Fatalln("Error (connection.Write):", err)
    }

    log.Println("Data sent:", n)
}