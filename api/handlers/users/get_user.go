package handlers

import (
	"net/http"
	"time"

	"github.com/arielkeren/lyra/models"
	"github.com/arielkeren/lyra/utils"
	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
)

type GetUserResponse struct {
	Username        string    `json:"username"`
	CreatedAt       time.Time `json:"createdAt"`
	PackagesCreated []string  `json:"packagesCreated"`
}

func GetUser(w http.ResponseWriter, r *http.Request, db *mongo.Database) {
	userIDStr := utils.GetParam(r, "/users/")

	userID, err := bson.ObjectIDFromHex(userIDStr)
	if err != nil {
		utils.SendError(w, http.StatusBadRequest, "Invalid user ID")
		return
	}

	ctx, cancel := utils.GetContext()
	defer cancel()

	var user models.User
	err = db.Collection("users").FindOne(ctx, bson.M{"_id": userID}).Decode(&user)
	if err != nil {
		utils.SendError(w, http.StatusNotFound, "User not found")
		return
	}

	utils.Send(w, http.StatusOK, GetUserResponse{
		Username:        user.Username,
		CreatedAt:       user.CreatedAt,
		PackagesCreated: user.PackagesCreated,
	})
}
