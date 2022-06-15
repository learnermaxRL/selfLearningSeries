package main

import (
	"bufio"
	"fmt"
	"net"
)

func handleConnectionRead(conn net.Conn) {

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

func handleConnectionBufio(conn net.Conn) {
	scanner := bufio.NewScanner(conn)
	// scanner.Split("\n")
	// var words string
	scanner.Split(bufio.ScanWords)
	for scanner.Scan() {

		fmt.Println(scanner.Text())
		// fmt.Println("-")
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

		go handleConnectionBufio(conn)

	}
	defer ln.Close()
}

func main() {
	CreateServer("127.0.0.1", "8089")
}
