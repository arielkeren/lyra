package models

import (
	"time"

	"go.mongodb.org/mongo-driver/v2/bson"
)

type User struct {
	ID              bson.ObjectID `json:"id" bson:"_id,omitempty"`
	Username        string        `json:"username" bson:"username"`
	Email           string        `json:"email" bson:"email"`
	Password        string        `json:"-" bson:"password"`
	CreatedAt       time.Time     `json:"createdAt" bson:"created_at"`
	PackagesCreated []string      `json:"packagesCreated" bson:"packages_created"`
}
