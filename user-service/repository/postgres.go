package repository

import (
	"context"
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"time"
	"user-service/domain"

	"github.com/jackc/pgx/v4/pgxpool"
)

type PostgresRepository struct {
	DB *pgxpool.Pool
}

func NewPostgres(db *pgxpool.Pool) *PostgresRepository {
	return &PostgresRepository{
		DB: db,
	}
}

func (r *PostgresRepository) CreateUser(user domain.User) (domain.User, error) {
	query := "INSERT INTO user (Name, Email, Status, Sex, Password, CreateAt) VALUES ($1, $2, $3, $4, $5, $6)"
	err := r.DB.QueryRow(context.Background(), query, user.Name, user.Email, user.Status, user.Sex, encrypt(user.Password), datetime_converter()).Scan(&user.ID)
	if err != nil {
		return domain.User{}, fmt.Errorf("error execute insert query")
	}
	return user, nil
}

func (r *PostgresRepository) UpdateUser(user domain.User) (domain.User, error) {
	query := "UPDATE user SET name=$2, Status=$3, Sex=$4, Password=$5, UpdateAt=$6"
	_, err := r.DB.Exec(context.Background(), query, user.Name, user.Status, user.Sex, encrypt(user.Password), datetime_converter())
	if err != nil {
		return domain.User{}, fmt.Errorf("error execute update query")
	}
	return user, nil
}

func (r *PostgresRepository) DeleteUser(userId int) error {
	query := "Update user SET Status=$1"
	_, err := r.DB.Exec(context.Background(), query, 2)
	return err
}

func (r *PostgresRepository) GetUserByNameAndPassword(name string, password string) (domain.User, error) {
	var user domain.User
	query := "SELECT ID, name, Status FROM user WHERE name=$1 and password=$2"
	err := r.DB.QueryRow(context.Background(), query, name, encrypt(password)).Scan(&user.ID, &user.Name, &user.Status)
	if err != nil {
		return domain.User{}, err
	}
	return user, nil
}

func (r *PostgresRepository) GetUser(userId int) (domain.User, error) {
	var user domain.User
	query := "SELECT ID, name, Status FROM user WHERE ID=$1"
	err := r.DB.QueryRow(context.Background(), query, userId).Scan(&user.ID, &user.Name, &user.Status)
	if err != nil {
		return domain.User{}, err
	}
	return user, nil
}

func encrypt(data string) string {
	hash := md5.New()
	hash.Write([]byte(data))
	hashbyte := hash.Sum(nil)
	hashString := hex.EncodeToString(hashbyte)
	return hashString
}

func datetime_converter() string {
	currentTime := time.Now()
	timeString := currentTime.Format("2006-01-02 15:04:05")
	return timeString
}
