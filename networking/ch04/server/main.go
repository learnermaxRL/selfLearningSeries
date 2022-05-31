package main

import (
	"fmt"
	"net"
)

func handleConnection(conn net.Conn) {

	out := make([]byte, 24)
	for {
		_, err := conn.Read(out)
		if err != nil {
			fmt.Println(err)
			return
		}
		fmt.Println(string(out))
	}

}

func CreateServer(addr, port string) {

	addr_c := addr + ":" + port
	fmt.Println(addr_c, "========")

	ln, err := net.Listen("tcp", addr_c)

	fmt.Println(ln.Addr().String())
	if err != nil {
		fmt.Println(err)
		return

	}
	fmt.Println("Server Started")
	for {
		conn, err := ln.Accept()
		if err != nil {
			// handle error
		}

		go handleConnection(conn)

	}
	defer ln.Close()
}

func main() {
	CreateServer("127.0.0.1", "8089")
}
