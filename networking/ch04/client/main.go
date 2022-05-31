// this will send something ro server

package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
	"strings"
)

func CreateClient(addr, port string) {

	addr_c := addr + ":" + port
	fmt.Println(addr_c)
	conn, err := net.Dial("tcp", addr_c)
	reader := bufio.NewReader(os.Stdin)
	fmt.Println("-------------------------")
	// defer conn.Close()

	if err != nil {
		fmt.Println(err)
		return

	}
	fmt.Println("======== Client Started ========")
	for {

		text, _ := reader.ReadString('\n')
		// convert CRLF to LF
		text = strings.Replace(text, "\n", "", -1)
		b := []byte(text)
		_, err := conn.Write(b)
		if err != nil {
			fmt.Println(err)
			continue
		}
		// out:=fmt.Sprintln("Server sent",port)
		// fmt.Println("recieved")

	}

}

func main() {
	CreateClient("127.0.0.1", "8089")
}
