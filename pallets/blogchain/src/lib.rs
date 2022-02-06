#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		inherent::Vec,
		pallet_prelude::*,
		sp_runtime::traits::Hash,
		traits::{tokens::ExistenceRequirement, Currency},
		transactional,
	};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId>;

		#[pallet::constant]
		type BlogPostMinBytes: Get<u32>;

		#[pallet::constant]
		type BlogPostMaxBytes: Get<u32>;

		#[pallet::constant]
		type BlogPostCommentMinBytes: Get<u32>;

		#[pallet::constant]
		type BlogPostCommentMaxBytes: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct BlogPost<T: Config> {
		pub content: Vec<u8>,
		pub author: <T as frame_system::Config>::AccountId,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct BlogPostComment<T: Config> {
		pub content: Vec<u8>,
		pub blog_post_id: T::Hash,
		pub author: <T as frame_system::Config>::AccountId,
	}

	/// Storage Map for BlogPosts by BlogPostId (Hash) to a BlogPost
	#[pallet::storage]
	#[pallet::getter(fn blog_posts)]
	pub(super) type BlogPosts<T: Config> = StorageMap<_, Twox64Concat, T::Hash, BlogPost<T>>;

	/// Storage Map from BlogPostId (Hash) to a list of BlogPostComments for this BlogPost
	#[pallet::storage]
	#[pallet::getter(fn blog_post_comments)]
	pub(super) type BlogPostComments<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, Vec<BlogPostComment<T>>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BlogPostCreated(Vec<u8>, T::AccountId, T::Hash),
		BlogPostCommentCreated(Vec<u8>, T::AccountId, T::Hash),
		Tipped(T::AccountId, T::Hash),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		BlogPostNotEnoughBytes,
		BlogPostTooManyBytes,
		BlogPostCommentNotEnoughBytes,
		BlogPostCommentTooManyBytes,
		BlogPostNotFound,
		TipperIsAuthor,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10000)]
		#[transactional]
		pub fn create_blog_post(origin: OriginFor<T>, content: Vec<u8>) -> DispatchResult {
			let author = ensure_signed(origin)?;

			ensure!(
				(content.len() as u32) > T::BlogPostMinBytes::get(),
				<Error<T>>::BlogPostNotEnoughBytes
			);

			ensure!(
				(content.len() as u32) < T::BlogPostMaxBytes::get(),
				<Error<T>>::BlogPostTooManyBytes
			);

			let blog_post = BlogPost { content: content.clone(), author: author.clone() };

			let blog_post_id = T::Hashing::hash_of(&blog_post);

			<BlogPosts<T>>::insert(blog_post_id, blog_post);

			let comments_vec: Vec<BlogPostComment<T>> = Vec::new();
			<BlogPostComments<T>>::insert(blog_post_id, comments_vec);

			Self::deposit_event(Event::BlogPostCreated(content, author, blog_post_id));

			Ok(())
		}

		#[pallet::weight(5000)]
		pub fn create_blog_post_comment(
			origin: OriginFor<T>,
			content: Vec<u8>,
			blog_post_id: T::Hash,
		) -> DispatchResult {
			let comment_author = ensure_signed(origin)?;

			ensure!(
				(content.len() as u32) > T::BlogPostMinBytes::get(),
				<Error<T>>::BlogPostCommentNotEnoughBytes
			);

			ensure!(
				(content.len() as u32) < T::BlogPostMaxBytes::get(),
				<Error<T>>::BlogPostCommentTooManyBytes
			);

			let blog_post_comment = BlogPostComment {
				author: comment_author.clone(),
				content: content.clone(),
				blog_post_id: blog_post_id.clone(),
			};

			<BlogPostComments<T>>::mutate(blog_post_id, |comments| match comments {
				None => Err(()),
				Some(vec) => {
					vec.push(blog_post_comment);
					Ok(())
				},
			})
			.map_err(|_| <Error<T>>::BlogPostNotFound)?;

			Self::deposit_event(Event::BlogPostCommentCreated(
				content,
				comment_author,
				blog_post_id,
			));

			Ok(())
		}

		#[pallet::weight(500)]
		pub fn tip_blog_post(
			origin: OriginFor<T>,
			blog_post_id: T::Hash,
			amount: <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance,
		) -> DispatchResult {
			let tipper = ensure_signed(origin)?;

			let blog_post = Self::blog_posts(&blog_post_id).ok_or(<Error<T>>::BlogPostNotFound)?;
			let blog_post_author = blog_post.author;

			ensure!(tipper != blog_post_author, <Error<T>>::TipperIsAuthor);

			T::Currency::transfer(
				&tipper,
				&blog_post_author,
				amount,
				ExistenceRequirement::KeepAlive,
			)?;

			Self::deposit_event(Event::Tipped(tipper, blog_post_id));

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {}
}
