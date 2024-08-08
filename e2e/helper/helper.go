package helper

import (
	"os"
	"testing"
)

func InitTest(t *testing.T) {
	err := os.Setenv("STORAGE_EMULATOR_HOST", "localhost:8000")
	if err != nil {
		t.Fatalf("Failed to set environment variable: %v", err)
	}
}
