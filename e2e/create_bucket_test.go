package main

import (
	"context"
	"testing"

	"cloud.google.com/go/storage"
	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
	"github.com/yuk1ty/cloud-storage-emulator-e2e/helper"
)

func TestCreateBucket(t *testing.T) {
	helper.InitTest(t)

	t.Run("Create one bucket without any error", func(t *testing.T) {
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

		// Act
		bucket := client.Bucket(testBucketName)
		if err := bucket.Create(ctx, "test-project", nil); err != nil {
			t.Fatalf("Failed to create bucket: %v", err)
		}

		// Assert
		assert.Nil(err)
	})

	t.Run("Create one bucket and create again then get 409 conflict", func(t *testing.T) {
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

		// Act
		bucket := client.Bucket(testBucketName)
		if err := bucket.Create(ctx, "test-project", nil); err != nil {
			t.Fatalf("Failed to create bucket: %v", err)
		}
		err = bucket.Create(ctx, "test-project", nil)

		// Assert
		assert.NotNilf(err, "error message: %v", err)
	})
}
