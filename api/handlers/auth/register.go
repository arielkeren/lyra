package handlers

import (
	"net/http"
	"time"

	"github.com/arielkeren/lyra/models"
	"github.com/arielkeren/lyra/utils"
	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
	"golang.org/x/crypto/bcrypt"
)

type RegisterRequest struct {
	Username string `json:"username"`
	Email    string `json:"email"`
	Password string `json:"password"`
}

type RegisterResponse struct {
	ID    string `json:"id"`
	Token string `json:"token"`
}

func Register(w http.ResponseWriter, r *http.Request, db *mongo.Database) {
	req, ok := utils.GetBody[RegisterRequest](w, r)
	if !ok {
		return
	}

	// Validate required fields
	if req.Username == "" || req.Email == "" || req.Password == "" {
		utils.SendError(w, http.StatusBadRequest, "Username, email and password are required")
		return
	}

	// Hash the password
	hashedPassword, err := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	if err != nil {
		utils.SendError(w, http.StatusInternalServerError, "Failed to hash password")
		return
	}

	// Create user document
	user := models.User{
		Username:        req.Username,
		Email:           req.Email,
		Password:        string(hashedPassword),
		CreatedAt:       time.Now(),
		PackagesCreated: []string{},
	}

	ctx, cancel := utils.GetContext()
	defer cancel()

	// Check if user already exists
	var existingUser models.User
	err = db.Collection("users").FindOne(ctx, bson.M{"email": req.Email}).Decode(&existingUser)

	if err == nil {
		utils.SendError(w, http.StatusConflict, "User with this email already exists")
		return
	} else if err != mongo.ErrNoDocuments {
		utils.SendError(w, http.StatusInternalServerError, "Database error")
		return
	}

	// Insert the user
	result, err := db.Collection("users").InsertOne(ctx, user)
	if err != nil {
		utils.SendError(w, http.StatusInternalServerError, "Failed to create user")
		return
	}

	// Set the user ID
	user.ID = result.InsertedID.(bson.ObjectID)

	// Generate JWT token
	token, err := utils.GenerateJWT(user.ID, user.Username, user.Email)
	if err != nil {
		utils.SendError(w, http.StatusInternalServerError, "Failed to generate token")
		return
	}

	utils.Send(w, http.StatusCreated, RegisterResponse{
		ID:    user.ID.Hex(),
		Token: token,
	})
}
