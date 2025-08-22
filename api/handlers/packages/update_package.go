package handlers

import (
	"net/http"
	"strings"
	"time"

	"github.com/arielkeren/lyra/models"
	"github.com/arielkeren/lyra/utils"
	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
)

type UpdatePackageRequest struct {
	Description string               `json:"description"`
	Version     string               `json:"version"`
	Files       []models.PackageFile `json:"files"`
}

func UpdatePackage(w http.ResponseWriter, r *http.Request, db *mongo.Database) {
	req, ok := utils.GetBody[UpdatePackageRequest](w, r)
	if !ok {
		return
	}

	packageName := utils.GetParam(r, "/packages/")

	userID, ok := utils.GetUserID(w, r)
	if !ok {
		return
	}

	ctx, cancel := utils.GetContext()
	defer cancel()

	collection := db.Collection("packages")

	// Verify ownership
	var pkg models.Package
	err := collection.FindOne(ctx, bson.M{"name": packageName}).Decode(&pkg)
	if err != nil {
		utils.SendError(w, http.StatusNotFound, "Package not found")
		return
	}

	if pkg.Owner != userID {
		utils.SendError(w, http.StatusForbidden, "Not authorized to update this package")
		return
	}

	// Build update query - only update non-empty fields
	updateFields := bson.M{}

	if strings.TrimSpace(req.Description) != "" {
		updateFields["description"] = req.Description
	}

	if strings.TrimSpace(req.Version) != "" {
		updateFields["version"] = req.Version
	}

	if len(req.Files) > 0 {
		updateFields["files"] = req.Files
	}

	if len(updateFields) == 0 {
		utils.SendError(w, http.StatusBadRequest, "No fields to update")
		return
	}

	updateFields["updated_at"] = time.Now()

	update := bson.M{"$set": updateFields}
	_, err = collection.UpdateOne(ctx, bson.M{"name": packageName}, update)
	if err != nil {
		utils.SendError(w, http.StatusInternalServerError, "Failed to update package")
		return
	}

	utils.SendEmpty(w)
}
