package handlers

import (
	"net/http"

	"github.com/arielkeren/lyra/models"
	"github.com/arielkeren/lyra/utils"
	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
)

func DeletePackage(w http.ResponseWriter, r *http.Request, db *mongo.Database) {
	packageName := utils.GetParam(r, "/packages/")

	userID, ok := utils.GetUserID(w, r)
	if !ok {
		return
	}

	ctx, cancel := utils.GetContext()
	defer cancel()

	// First, verify the package exists and user owns it
	var pkg models.Package
	err := db.Collection("packages").FindOne(ctx, bson.M{"name": packageName}).Decode(&pkg)
	if err != nil {
		if err == mongo.ErrNoDocuments {
			utils.SendError(w, http.StatusNotFound, "Package not found")
		} else {
			utils.SendError(w, http.StatusInternalServerError, "Database error")
		}
		return
	}

	// Check ownership
	if pkg.Owner != userID {
		utils.SendError(w, http.StatusForbidden, "Not authorized to delete this package")
		return
	}

	// Delete the package
	_, err = db.Collection("packages").DeleteOne(ctx, bson.M{"name": packageName})
	if err != nil {
		utils.SendError(w, http.StatusInternalServerError, "Failed to delete package")
		return
	}

	// Remove package ID from user's packages_created array
	db.Collection("users").UpdateOne(ctx,
		bson.M{"_id": userID},
		bson.M{"$pull": bson.M{"packages_created": packageName}},
	)

	// Return 204 No Content for successful deletion
	utils.SendEmpty(w)
}
