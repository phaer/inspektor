package models

import (
	"encoding/json"
	"inspektor/utils"

	"gorm.io/datatypes"
	"gorm.io/gorm"
)

type Session struct {
	gorm.Model
	ObjectID    uint           `json:"objectID"`
	UserID      uint           `json:"-"`
	Meta        datatypes.JSON `json:"meta"`
	SessionMeta *SessionMeta   `gorm:"-" json:"-"`
}

type SessionMeta struct {
	Type             string   `json:"type"`
	PostgresPassword string   `json:"postgresPassword"`
	PostgresUsername string   `json:"postgresUsername"`
	TempRoles        []string `json:"tempRoles"`
	ExpiresAt        int64    `json:"expiresAt"`
}

func (s *Session) UnmarshalMeta() {
	meta := &SessionMeta{}
	if len(s.Meta) != 0 {
		json.Unmarshal(s.Meta, &meta)
	}
	s.SessionMeta = meta
}

func (s *Session) MarshalMeta() {
	buf, err := json.Marshal(s.SessionMeta)
	utils.Check(err)
	s.Meta = buf
}
