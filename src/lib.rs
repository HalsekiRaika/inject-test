pub trait Repository: 'static + Send + Sync {
    fn create(&self) -> Result<(), u64>;
}
#[derive(Clone)]
pub struct Pool;

#[derive(Clone)]
pub struct DataRepository(Pool);
impl Repository for DataRepository {
    fn create(&self) -> Result<(), u64> {
        Ok(())
    }
}
pub trait DependOnRepository: 'static + Send + Sync {
    type Repository: Repository;
    fn repository(&self) -> &Self::Repository;
}

pub mod simple {
    use crate::{DependOnRepository, DataRepository, Pool, Repository};

    pub trait CreateSimpleDataService: 'static + Send + Sync
        + DependOnRepository
    {
        fn create(&self, _obj: String) -> Result<u64, u64> {
            self.repository().create()?;
            Ok(1_u64)
        }
    }
    impl<T> CreateSimpleDataService for T
        where T: DependOnRepository {}
    pub trait DependOnCreateSimpleDataService: 'static + Send + Sync {
        type CreateSimpleDataService: CreateSimpleDataService;
        fn create_simple_data_service(&self) -> &Self::CreateSimpleDataService;
    }

    pub struct SimpleHandler {
        repo: DataRepository
    }
    impl SimpleHandler {
        pub fn init() -> Self {
            Self { repo: DataRepository(Pool) }
        }
    }
    impl DependOnRepository for SimpleHandler {
        type Repository = DataRepository;
        fn repository(&self) -> &Self::Repository {
            &self.repo
        }
    }
    impl DependOnCreateSimpleDataService for SimpleHandler {
        type CreateSimpleDataService = Self;
        fn create_simple_data_service(&self) -> &Self::CreateSimpleDataService {
            self
        }
    }
}

pub mod interactor {
    use crate::{DependOnRepository, Repository, DataRepository, Pool};

    pub trait CreateDataService: 'static + Send + Sync
        + DependOnRepository
    {
        fn create(&self, obj: String) -> Result<u64, u64>;
    }
    pub struct CreateDataInteractor<T> {
        handler: T
    }
    impl<T> DependOnRepository for CreateDataInteractor<T>
        where T: Repository
    {
        type Repository = T;
        fn repository(&self) -> &Self::Repository {
            &self.handler
        }
    }
    impl<T> CreateDataService for CreateDataInteractor<T>
        where T: Repository
    {
        fn create(&self, _obj: String) -> Result<u64, u64> {
            self.handler.create()?;
            Ok(1_u64)
        }
    }
    pub trait DependOnCreateDataService: 'static + Send + Sync {
        type CreateDataService: CreateDataService;
        fn create_data_service(&self) -> &Self::CreateDataService;
    }
    
    pub struct InteractionHandler {
        repo: DataRepository,
        interactor: CreateDataInteractor<DataRepository>
    }
    impl InteractionHandler {
        pub fn init() -> Self {
            let repo = DataRepository(Pool);
            Self { repo: repo.clone(), interactor: CreateDataInteractor { handler: repo } }
        }
    }
    impl DependOnRepository for InteractionHandler {
        type Repository = DataRepository;
        fn repository(&self) -> &Self::Repository {
            &self.repo
        }
    }
    impl DependOnCreateDataService for InteractionHandler {
        type CreateDataService = CreateDataInteractor<DataRepository>;
        fn create_data_service(&self) -> &Self::CreateDataService {
            &self.interactor
        }
    }
}

pub mod mixin {
    use crate::{DependOnRepository, DataRepository, Pool, Repository};

    pub trait CreateUserService<T>: 'static + Sync + Send 
        where T: ?Sized + 'static + Sync + Send
    {
        fn create(this: &T, id: String) -> Result<u64, u64>;
    }

    pub trait MixinCreateUserService: 'static + Sync + Send 
        + DependOnRepository
    {
        type Definite: CreateUserService<Self>;

        fn create(&self, id: String) -> Result<u64, u64> {
            Self::Definite::create(self, id)
        }
    }

    pub struct UserService;

    impl<T> CreateUserService<T> for UserService
        where T: DependOnRepository
    {
        fn create(this: &T, _id: String) -> Result<u64, u64> {
            this.repository().create()?;
            Ok(1_u64)
        }
    }

    pub struct MixinHandler {
        repo: DataRepository
    }

    impl MixinHandler {
        pub fn init() -> Self {
            Self { repo: DataRepository(Pool) }
        }
    }

    impl DependOnRepository for MixinHandler {
        type Repository = DataRepository;
        fn repository(&self) -> &Self::Repository {
            &self.repo
        }
    }

    impl MixinCreateUserService for MixinHandler {
        type Definite = UserService;
    }
}