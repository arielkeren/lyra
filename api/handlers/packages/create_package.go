package handlers

import (
	"net/http"
	"time"

	"github.com/arielkeren/lyra/models"
	"github.com/arielkeren/lyra/utils"
	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
)

type CreatePackageRequest struct {
	Name        string               `json:"name"`
	Description string               `json:"description"`
	Version     string               `json:"version"`
	Files       []models.PackageFile `json:"files"`
}

func CreatePackage(w http.ResponseWriter, r *http.Request, db *mongo.Database) {
	req, ok := utils.GetBody[CreatePackageRequest](w, r)
	if !ok {
		return
	}

	// Validate required fields
	if req.Name == "" {
		utils.SendError(w, http.StatusBadRequest, "Name is required")
		return
	}

	userID, ok := utils.GetUserID(w, r)
	if !ok {
		return
	}

	// Create package document
	pkg := models.Package{
		Name:        req.Name,
		Description: req.Description,
		Version:     req.Version,
		Owner:       userID,
		Files:       req.Files,
		CreatedAt:   time.Now(),
		UpdatedAt:   time.Now(),
	}

	ctx, cancel := utils.GetContext()
	defer cancel()

	// Insert the package
	result, err := db.Collection("packages").InsertOne(ctx, pkg)
	if err != nil {
		utils.SendError(w, http.StatusInternalServerError, "Failed to create package")
		return
	}

	// Update user's packages_created array
	_, err = db.Collection("users").UpdateOne(ctx, bson.M{"_id": userID}, bson.M{
		"$addToSet": bson.M{"packages_created": result.InsertedID},
	})
	if err != nil {
		utils.SendError(w, http.StatusInternalServerError, "Failed to update user")
		return
	}

	utils.SendEmpty(w)
}
