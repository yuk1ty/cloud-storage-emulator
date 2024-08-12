package main

import (
	"context"
	"testing"

	"cloud.google.com/go/storage"
	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
	"github.com/yuk1ty/cloud-storage-emulator-e2e/helper"
)

func TestDeleteBucket(t *testing.T) {
	helper.InitTest(t)

	t.Run("Delete one bucket without any error", func(t *testing.T) {
		// Arrange
		ctx := context.Background()

		assert := assert.New(t)

		client, err := storage.NewClient(ctx)
		if err != nil {
			t.Fatalf("Failed to create client: %v", err)
		}

		testUniqID, err := uuid.NewRandom()
		if err != nil {
			t.Fatalf("Failed to generate a new UUID")
		}
		testBucketName := "test-bucket-" + testUniqID.String()
		bucket := client.Bucket(testBucketName)
		if err := bucket.Create(ctx, "test-project", nil); err != nil {
			t.Fatalf("Failed to create bucket: %v", err)
		}

		// Act
		if err := bucket.Delete(ctx); err != nil {
			t.Fatalf("Failed to delete bucket: %v", err)
		}

		// Assert
		assert.Nil(err)
	})
}
