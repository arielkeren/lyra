package utils

import (
	"context"
	"encoding/json"
	"net/http"
	"strings"
	"time"

	"go.mongodb.org/mongo-driver/v2/bson"
)

func GetContext() (context.Context, context.CancelFunc) {
	return context.WithTimeout(context.Background(), 5*time.Second)
}

func GetUserID(w http.ResponseWriter, r *http.Request) (bson.ObjectID, bool) {
	userIDStr, ok := r.Context().Value("id").(string)
	if !ok {
		SendError(w, http.StatusUnauthorized, "User ID not found in context")
		return bson.ObjectID{}, false
	}

	userID, err := bson.ObjectIDFromHex(userIDStr)
	if err != nil {
		SendError(w, http.StatusBadRequest, "Invalid user ID")
		return bson.ObjectID{}, false
	}

	return userID, true
}

func GetParam(r *http.Request, route string) string {
	return strings.TrimPrefix(r.URL.Path, route)
}

func GetBody[T any](w http.ResponseWriter, r *http.Request) (T, bool) {
	var body T
	if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
		SendError(w, http.StatusBadRequest, "Invalid JSON")
		return body, false
	}
	return body, true
}

func Send(w http.ResponseWriter, statusCode int, data any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(statusCode)
	json.NewEncoder(w).Encode(data)
}

func SendEmpty(w http.ResponseWriter) {
	w.WriteHeader(http.StatusNoContent)
}

func SendError(w http.ResponseWriter, statusCode int, message string) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(statusCode)
	json.NewEncoder(w).Encode(map[string]string{"error": message})
}
