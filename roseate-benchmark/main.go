package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"os/exec"
	"strings"
	"time"
)

func main() {
	imageViewers := []string{
		"roseate",
		"roseate_old",
	}

	file := os.Args[1]

	for _, viewer := range imageViewers {
		start := time.Now()

		cmd := exec.Command(viewer, file)

		stdout, _ := cmd.StderrPipe()

		if err := cmd.Start(); err != nil {
			log.Fatal(err)
		}

		scanner := bufio.NewScanner(stdout)

		go func() {
			for scanner.Scan() {
				line := scanner.Text()

				if strings.Contains(line, "The image has been scheduled to resize to the window size.") {
					elapsed := time.Since(start)
					fmt.Printf("%s: %s\n", viewer, elapsed)

					cmd.Process.Kill()

					break
				} else if strings.Contains(line, "Failed to apply optimization") {
					fmt.Printf("%s: Failed\n", viewer)

					cmd.Process.Kill()

					break
				}
			}
		}()

		cmd.Wait()
	}
}
