pub const TEMPLATE_DB_CONNECT: &str = r#"
        <?php

        namespace {{ php.namespace }}!;

        use PDO;

        class Database
        {
            private static ?Database \$instance = null;
            private PDO \$pdo;

            private function __construct()
            {
                \$dsn = "mysql:host={{storage.db_host}};dbname={{storage.db_name}};charset={{storage.db_charset}}";

                \$options = [
                    PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
                    PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC,
                    PDO::ATTR_EMULATE_PREPARES => false,
                ];

                \$this->pdo = new PDO(\$dsn, "{{storage.db_login}}", "{{storage.db_password}}", \$options);

            }

            public static function connection(): PDO
            {
                if (self::\$instance === null) {
                    self::\$instance = new self();
                }
                return self::\$instance->pdo;
            }
        }
        "#;

pub const TEMPLATE_MAIN_CLASS: &str = r#"

    class {{php.entity}}
    {
        private static ?Create $create = null;
        private static ?Delete $delete = null;

        static function create(): Create
        {
            return self::$create ??= new Create();
        }

        static function delete(): Delete
        {
            return self::$delete ??= new Delete();
        }

        static function find(): Delete
        {
            return self::$delete ??= new Delete();
        }

    }"#;

pub const TEMPLATE_ENUM_CONDITIONS: &str = r#"
    enum Condition
    {
        case EQ; ==
        case NOT_EQ; !=
        case GT; >
        case GT_EQ; >=
        case LESS; <
        case LESS_EQ; =<
        case LIKE; LIKE
        case NOT_LIKE; NOT LIKE
        case IS_NULL; IS NULL
        case IS NOT NULL; IS NOT NULL
    }
"#;

pub const TEMPLATE_CREATE: &str = r#"

    class Create
    {
        private const string TABLE_NAME = '{{db.table}}';
        private ?PDO $connection;

        /** @var array<string, scalar|null> */
        private array $fields = [];

        public function __construct(PDO $connection)
        {
            $this->connection = $connection;
        }

        protected function setField(string $name, string|int|float|bool|null $value): void
        {
            Tools::assertValidIdentifier($name);

            $this->fields[$name] = $value;
        }

        public function save(): void
        {
            if ($this->fields === []) {
                throw new LogicException('Невозможно выполнить INSERT без полей');
            }

            $connection =  $this->connection
                ?? throw new LogicException('PDO-соединение не установлено. Вызовите Create::setConnection()');

            $columns = array_keys($this->fields);
            $quotedColumns = array_map(Tools::quoteIdentifier(...), $columns);
            $placeholders = array_map(
                static fn(string $column): string => ':' . $column,
                $columns,
            );

            $sql = sprintf(
                'INSERT INTO %s (%s) VALUES (%s)',
                Tools::quoteIdentifier(static::TABLE_NAME),
                implode(', ', $quotedColumns),
                implode(', ', $placeholders),
            );

            $statement = $connection->prepare($sql);
            if ($statement === false || !$statement->execute($this->fields)) {
                throw new RuntimeException('Не удалось выполнить INSERT');
            }

            $this->fields = [];
        }

        {{php.methods}}
    }
"#;
