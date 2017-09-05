package main;

import "net"
import "log"


func main() {
    packet_connection, err:=net.ListenPacket("udp6", ":1234")
    if err!=nil{
        log.Fatalln("Error (net.ListenPacket):", err)
    }
    defer packet_connection.Close()
    log.Println("Listening on:", packet_connection.LocalAddr())

    buffer:=make([]byte, 1000)
    for{
        n, addr, err:=packet_connection.ReadFrom(buffer)
        if err!=nil{
            log.Println("Error (packet_connection.ReadFrom):", err)
            continue
        }

        log.Printf("%s->%s: [%d] %s", addr, packet_connection.LocalAddr(), n, string(buffer[:n]))
    }
}