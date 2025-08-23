package handlers

import (
	"net/http"
	"strings"

	"github.com/arielkeren/lyra/models"
	"github.com/arielkeren/lyra/utils"
	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
	"golang.org/x/crypto/bcrypt"
)

type UpdateUserRequest struct {
	Username string `json:"username"`
	Email    string `json:"email"`
	Password string `json:"password"`
}

type UpdateUserResponse struct {
	Token string `json:"token"`
}

func UpdateUser(w http.ResponseWriter, r *http.Request, db *mongo.Database) {
	req, ok := utils.GetBody[UpdateUserRequest](w, r)
	if !ok {
		return
	}

	userID, ok := utils.GetUserID(w, r)
	if !ok {
		return
	}

	ctx, cancel := utils.GetContext()
	defer cancel()

	// Build update query - only update non-empty fields
	updateFields := bson.M{}

	if strings.TrimSpace(req.Username) != "" {
		updateFields["username"] = req.Username
	}

	if strings.TrimSpace(req.Email) != "" {
		// Check if another user already has this email
		var existingUser models.User
		err := db.Collection("users").FindOne(ctx, bson.M{
			"email": req.Email,
			"_id":   bson.M{"$ne": userID}, // Exclude current user
		}).Decode(&existingUser)

		if err == nil {
			// User found with this email
			utils.SendError(w, http.StatusConflict, "Email already in use")
			return
		} else if err != mongo.ErrNoDocuments {
			// Database error (not "no documents found")
			utils.SendError(w, http.StatusInternalServerError, "Failed to check email availability")
			return
		}

		updateFields["email"] = req.Email
	}

	if strings.TrimSpace(req.Password) != "" {
		hashedPassword, err := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
		if err != nil {
			utils.SendError(w, http.StatusInternalServerError, "Failed to hash password")
			return
		}
		updateFields["password"] = string(hashedPassword)
	}

	if len(updateFields) == 0 {
		utils.SendError(w, http.StatusBadRequest, "No fields to update")
		return
	}

	_, err := db.Collection("users").UpdateOne(ctx, bson.M{"_id": userID}, bson.M{"$set": updateFields})
	if err != nil {
		utils.SendError(w, http.StatusInternalServerError, "Failed to update user")
		return
	}

	// Get the updated user data
	var updatedUser models.User
	err = db.Collection("users").FindOne(ctx, bson.M{"_id": userID}).Decode(&updatedUser)
	if err != nil {
		utils.SendError(w, http.StatusInternalServerError, "Failed to retrieve updated user")
		return
	}

	// Generate new JWT with updated information
	token, err := utils.GenerateJWT(updatedUser.ID, updatedUser.Username, updatedUser.Email)
	if err != nil {
		utils.SendError(w, http.StatusInternalServerError, "Failed to generate token")
		return
	}

	utils.Send(w, http.StatusOK, UpdateUserResponse{
		Token: token,
	})
}
