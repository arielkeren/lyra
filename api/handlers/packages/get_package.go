package handlers

import (
	"net/http"
	"time"

	"github.com/arielkeren/lyra/models"
	"github.com/arielkeren/lyra/utils"
	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
)

type GetPackageResponse struct {
	Description string               `json:"description"`
	Version     string               `json:"version"`
	Owner       bson.ObjectID        `json:"owner"`
	Files       []models.PackageFile `json:"files"`
	CreatedAt   time.Time            `json:"createdAt"`
	UpdatedAt   time.Time            `json:"updatedAt"`
}

func GetPackage(w http.ResponseWriter, r *http.Request, db *mongo.Database) {
	packageName := utils.GetParam(r, "/packages/")

	ctx, cancel := utils.GetContext()
	defer cancel()

	var pkg models.Package
	err := db.Collection("packages").FindOne(ctx, bson.M{"name": packageName}).Decode(&pkg)
	if err != nil {
		utils.SendError(w, http.StatusNotFound, "Package not found")
		return
	}

	utils.Send(w, http.StatusOK, GetPackageResponse{
		Description: pkg.Description,
		Version:     pkg.Version,
		Owner:       pkg.Owner,
		Files:       pkg.Files,
		CreatedAt:   pkg.CreatedAt,
		UpdatedAt:   pkg.UpdatedAt,
	})
}
