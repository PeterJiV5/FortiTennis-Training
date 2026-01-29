-- Create training_templates table (global template library)
CREATE TABLE IF NOT EXISTS training_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    coach_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    content_type TEXT NOT NULL CHECK(content_type IN ('drill', 'exercise', 'warmup', 'cooldown', 'quiz', 'homework')),
    description TEXT,
    duration_minutes INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_by INTEGER NOT NULL,
    last_edited_by INTEGER,
    last_edited_at DATETIME,
    is_public BOOLEAN DEFAULT 1,
    FOREIGN KEY (coach_id) REFERENCES users(id),
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (last_edited_by) REFERENCES users(id)
);

-- Create session_training_links table (junction table)
CREATE TABLE IF NOT EXISTS session_training_links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    training_template_id INTEGER NOT NULL,
    order_index INTEGER NOT NULL,
    custom_notes TEXT,
    UNIQUE(session_id, order_index),
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (training_template_id) REFERENCES training_templates(id) ON DELETE CASCADE
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_training_templates_coach ON training_templates(coach_id);
CREATE INDEX IF NOT EXISTS idx_training_templates_created_by ON training_templates(created_by);
CREATE INDEX IF NOT EXISTS idx_session_training_links_session ON session_training_links(session_id);
CREATE INDEX IF NOT EXISTS idx_session_training_links_template ON session_training_links(training_template_id);
