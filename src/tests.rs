use crate::mock::*;
use crate::{pallet, Error, Event as StreamPaymentsEvent, Stream};
use frame_support::traits::OnInitialize;
use frame_support::{assert_noop, assert_ok};

fn last_event() -> StreamPaymentsEvent<Test> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let Event::StreamPayments(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .last()
        .unwrap()
}

const SPEND_RATE: u64 = INIT_BALANCE / 1000;

#[test]
fn genesis_config() {
    new_test_ext().execute_with(|| {
        assert_eq!(<pallet::Streams<Test>>::iter().count(), 0);
    });
}

#[test]
fn open_stream() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(StreamPayments::open_stream(
            Origin::signed(A),
            B,
            SPEND_RATE
        ));
        assert_eq!(
            *StreamPayments::streams(A),
            [Stream {
                target: B,
                spend_rate: SPEND_RATE
            }]
        );
        assert_eq!(
            last_event(),
            StreamPaymentsEvent::StreamOpened(A, B, SPEND_RATE)
        );
    });
}

#[test]
fn stream_limit_reached() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        for _ in 0..MAX_STREAMS {
            assert_ok!(StreamPayments::open_stream(
                Origin::signed(A),
                B,
                SPEND_RATE
            ));
        }
        assert_noop!(
            StreamPayments::open_stream(Origin::signed(A), B, SPEND_RATE),
            Error::<Test>::StreamLimitReached
        );
    });
}

#[test]
fn reflexive_stream() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            StreamPayments::open_stream(Origin::signed(A), A, SPEND_RATE),
            Error::<Test>::ReflexiveStream
        );
    });
}

#[test]
fn close_stream() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(StreamPayments::open_stream(
            Origin::signed(A),
            B,
            SPEND_RATE
        ));
        assert_ok!(StreamPayments::close_stream(Origin::signed(A), 0));
        assert_eq!(*StreamPayments::streams(A), []);
        assert_eq!(
            last_event(),
            StreamPaymentsEvent::StreamClosed(A, B, SPEND_RATE)
        );
    });
}

#[test]
fn stream_not_found() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            StreamPayments::close_stream(Origin::signed(A), 0),
            Error::<Test>::StreamNotFound
        );
    });
}

#[test]
fn payment_made() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(StreamPayments::open_stream(
            Origin::signed(A),
            B,
            SPEND_RATE
        ));

        // Tick the clock - step one block
        System::set_block_number(2);
        <StreamPayments as OnInitialize<u64>>::on_initialize(2);
        assert_eq!(
            last_event(),
            StreamPaymentsEvent::PaymentMade(A, B, SPEND_RATE)
        );
        assert_eq!(Balances::free_balance(A), INIT_BALANCE - SPEND_RATE);
        assert_eq!(Balances::free_balance(B), INIT_BALANCE + SPEND_RATE);

        // Let's add one more stream in the opposite direction
        assert_ok!(StreamPayments::open_stream(
            Origin::signed(B),
            A,
            SPEND_RATE * 10
        ));
        System::set_block_number(2);
        <StreamPayments as OnInitialize<u64>>::on_initialize(2);
        assert_eq!(
            Balances::free_balance(A),
            INIT_BALANCE - 2 * SPEND_RATE + 10 * SPEND_RATE
        );
        assert_eq!(
            Balances::free_balance(B),
            INIT_BALANCE + 2 * SPEND_RATE - 10 * SPEND_RATE
        );
    });
}

#[test]
fn stream_exhausted() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // Transferring the whole funds A owns
        assert_ok!(StreamPayments::open_stream(
            Origin::signed(A),
            B,
            INIT_BALANCE
        ));

        // Step two blocks - the second transfer should fail
        for i in [2, 3] {
            System::set_block_number(i);
            <StreamPayments as OnInitialize<u64>>::on_initialize(i);
        }
        assert_eq!(
            last_event(),
            StreamPaymentsEvent::StreamExhausted(A, B, INIT_BALANCE,)
        );
        assert_eq!(Balances::free_balance(A), 0);
        assert_eq!(Balances::free_balance(B), 2 * INIT_BALANCE);

        // Check if exhausted stream has been correctly removed
        assert_eq!(*StreamPayments::streams(A), []);
    });
}
