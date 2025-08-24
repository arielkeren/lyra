package models

import (
	"time"

	"go.mongodb.org/mongo-driver/v2/bson"
)

type PackageFile struct {
	Name    string `json:"name" bson:"name"`
	Content string `json:"content" bson:"content"`
}

type Package struct {
	ID          bson.ObjectID `json:"id" bson:"_id,omitempty"`
	Name        string        `json:"name" bson:"name"`
	Description string        `json:"description" bson:"description"`
	Version     string        `json:"version" bson:"version"`
	Owner       bson.ObjectID `json:"owner" bson:"owner"`
	Files       []PackageFile `json:"files" bson:"files"`
	CreatedAt   time.Time     `json:"createdAt" bson:"created_at"`
	UpdatedAt   time.Time     `json:"updatedAt" bson:"updated_at"`
}
