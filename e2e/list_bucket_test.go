package main

import (
	"context"
	"fmt"
	"os"
	"testing"

	"cloud.google.com/go/storage"
)

func TestCreateAndListBucket(t *testing.T) {
	// Arrange
	err := os.Setenv("STORAGE_EMULATOR_HOST", "localhost:8000")
	if err != nil {
		t.Fatalf("Failed to set environment variable: %v", err)
	}

	ctx := context.Background()
	client, err := storage.NewClient(ctx)
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}

	bucket := client.Bucket("test-bucket")
	if err := bucket.Create(ctx, "test-project", nil); err != nil {
		t.Fatalf("Failed to create bucket: %v", err)
	}

	// Act
	attrs, err := bucket.Attrs(ctx)
	if err != nil {
		t.Fatalf("Failed to get bucket attributes: %v", err)
	}

	// Assert
	fmt.Println(attrs.Name)
}
