package main

import (
	"context"
	"testing"

	"cloud.google.com/go/storage"
	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
	"github.com/yuk1ty/cloud-storage-emulator-e2e/helper"
)

func TestUpdateBucket(t *testing.T) {
	helper.InitTest(t)

	t.Run("Update one bucket without any error", func(t *testing.T) {
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
		attrs := storage.BucketAttrs{
			VersioningEnabled:     true,
			DefaultEventBasedHold: true,
		}
		if err := bucket.Create(ctx, "test-project", &attrs); err != nil {
			t.Fatalf("Failed to create bucket: %v", err)
		}

		// Act
		uattrs := storage.BucketAttrsToUpdate{
			VersioningEnabled:     false,
			DefaultEventBasedHold: false,
		}
		updated, err := bucket.Update(ctx, uattrs)
		if err != nil {
			t.Fatalf("Failed to create bucket: %v", err)
		}

		// Assert
		assert.Nil(err)
		assert.Equal(false, updated.VersioningEnabled)
		assert.Equal(false, updated.DefaultEventBasedHold)
	})

	t.Run("Update a non existing bucket then get 404 not found", func(t *testing.T) {
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
		attrs := storage.BucketAttrs{
			VersioningEnabled:     true,
			DefaultEventBasedHold: true,
		}
		if err := bucket.Create(ctx, "test-project", &attrs); err != nil {
			t.Fatalf("Failed to create bucket: %v", err)
		}

		// Act
		nonExist := client.Bucket("non-exist-bucket")
		uattrs := storage.BucketAttrsToUpdate{
			VersioningEnabled:     false,
			DefaultEventBasedHold: false,
		}
		_, err = nonExist.Update(ctx, uattrs)

		// Assert
		assert.NotNilf(err, "error message: %v", err)
	})
}
