package handlers

import (
	"net/http"

	"github.com/arielkeren/lyra/models"
	"github.com/arielkeren/lyra/utils"
	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
	"golang.org/x/crypto/bcrypt"
)

type LoginRequest struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}

type LoginResponse struct {
	ID    string `json:"id"`
	Token string `json:"token"`
}

func Login(w http.ResponseWriter, r *http.Request, db *mongo.Database) {
	req, ok := utils.GetBody[LoginRequest](w, r)
	if !ok {
		return
	}

	// Validate required fields
	if req.Email == "" || req.Password == "" {
		utils.SendError(w, http.StatusBadRequest, "Email and password are required")
		return
	}

	ctx, cancel := utils.GetContext()
	defer cancel()

	collection := db.Collection("users")

	// Find user by email
	var user models.User
	err := collection.FindOne(ctx, bson.M{"email": req.Email}).Decode(&user)
	if err != nil {
		utils.SendError(w, http.StatusUnauthorized, "Invalid email or password")
		return
	}

	// Check password
	err = bcrypt.CompareHashAndPassword([]byte(user.Password), []byte(req.Password))
	if err != nil {
		utils.SendError(w, http.StatusUnauthorized, "Invalid email or password")
		return
	}

	// Generate JWT token
	token, err := utils.GenerateJWT(user.ID, user.Username, user.Email)
	if err != nil {
		utils.SendError(w, http.StatusInternalServerError, "Failed to generate token")
		return
	}

	utils.Send(w, http.StatusOK, LoginResponse{
		ID:    user.ID.Hex(),
		Token: token,
	})
}
