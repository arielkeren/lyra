package handlers

import (
	"net/http"

	"github.com/arielkeren/lyra/models"
	"github.com/arielkeren/lyra/utils"
	"go.mongodb.org/mongo-driver/v2/bson"
	"go.mongodb.org/mongo-driver/v2/mongo"
)

type GetPackagesResponse struct {
	Packages []GetPackageResponse `json:"packages"`
}

func GetPackages(w http.ResponseWriter, r *http.Request, db *mongo.Database) {
	ctx, cancel := utils.GetContext()
	defer cancel()

	var pkgs []models.Package
	cursor, err := db.Collection("packages").Aggregate(ctx, []bson.M{
		{"$sample": bson.M{"size": 10}},
	})
	if err != nil {
		utils.SendError(w, http.StatusNotFound, "No packages found")
		return
	}
	defer cursor.Close(ctx)

	for cursor.Next(ctx) {
		var pkg models.Package
		if err := cursor.Decode(&pkg); err != nil {
			utils.SendError(w, http.StatusInternalServerError, "Failed to decode package")
			return
		}
		pkgs = append(pkgs, pkg)
	}

	if len(pkgs) == 0 {
		utils.SendError(w, http.StatusNotFound, "No packages found")
		return
	}

	var res []GetPackageResponse
	for _, pkg := range pkgs {
		res = append(res, GetPackageResponse{
			Description: pkg.Description,
			Version:     pkg.Version,
			Owner:       pkg.Owner,
			Files:       pkg.Files,
			CreatedAt:   pkg.CreatedAt,
			UpdatedAt:   pkg.UpdatedAt,
		})
	}

	utils.Send(w, http.StatusOK, GetPackagesResponse{
		Packages: res,
	})
}
