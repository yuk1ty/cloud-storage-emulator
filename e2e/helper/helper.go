package helper

import (
	"context"
	"log"
	"os"
	"testing"

	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/wait"
)

func LaunchTestContainer(ctx context.Context) (testcontainers.Container, error) {
	// TODO: https://java.testcontainers.org/supported_docker_environment/
	req := testcontainers.ContainerRequest{
		Image:        "cloud-storage-emulator:latest",
		Name:         "test-cloud-storage-emulator",
		ExposedPorts: []string{"8000/tcp"},
		WaitingFor:   wait.ForListeningPort("8000/tcp"),
	}
	emu, err := testcontainers.GenericContainer(ctx, testcontainers.GenericContainerRequest{
		ContainerRequest: req,
		Started:          true,
		Reuse:            true,
	})
	return emu, err
}

func TerminateTestContainer(emu testcontainers.Container, ctx context.Context) {
	if err := emu.Terminate(ctx); err != nil {
		log.Fatalf("Could not stop emulator: %s", err)
	}
}

func InitTest(t *testing.T) {
	err := os.Setenv("STORAGE_EMULATOR_HOST", "localhost:8000")
	if err != nil {
		t.Fatalf("Failed to set environment variable: %v", err)
	}
}
