use crate::DataTypeView;
use crate::MainWindowView;
use crate::ManualScanViewModelBindings;
use crate::ScanConstraintTypeView;
use crate::ValueCollectorViewModelBindings;
use crate::view_models::scanners::scan_constraint_converter::ScanConstraintConverter;
use slint::ComponentHandle;
use slint::SharedString;
use slint_mvvm::view_binding::ViewBinding;
use slint_mvvm::view_data_converter::ViewDataConverter;
use slint_mvvm_macros::create_view_bindings;
use squalr_engine::command_executors::engine_request_executor::EngineRequestExecutor;
use squalr_engine::engine_execution_context::EngineExecutionContext;
use squalr_engine_api::commands::scan::collect_values::scan_collect_values_request::ScanCollectValuesRequest;
use squalr_engine_api::commands::scan::hybrid::scan_hybrid_request::ScanHybridRequest;
use squalr_engine_api::commands::scan::new::scan_new_request::ScanNewRequest;
use squalr_engine_common::structures::data_types::data_type_deprecated::DataType;
use squalr_engine_common::structures::endian::Endian;
use squalr_engine_common::structures::scanning::scan_filter_parameters::ScanFilterParameters;
use squalr_engine_common::structures::values::anonymous_value::AnonymousValue;
use std::sync::Arc;

pub struct ManualScanViewModel {
    _view_binding: ViewBinding<MainWindowView>,
    _engine_execution_context: Arc<EngineExecutionContext>,
}

impl ManualScanViewModel {
    pub fn new(
        view_binding: ViewBinding<MainWindowView>,
        engine_execution_context: Arc<EngineExecutionContext>,
    ) -> Self {
        let view: ManualScanViewModel = ManualScanViewModel {
            _view_binding: view_binding.clone(),
            _engine_execution_context: engine_execution_context.clone(),
        };

        create_view_bindings!(view_binding, {
            ManualScanViewModelBindings => {
                on_new_scan(data_type: DataTypeView) -> [engine_execution_context] -> Self::on_new_scan,
                on_start_scan(scan_constraint: ScanConstraintTypeView, scan_value: SharedString) -> [engine_execution_context] -> Self::on_start_scan,
            },
            ValueCollectorViewModelBindings => {
                on_collect_values() -> [engine_execution_context] -> Self::on_collect_values,
            },
        });

        view
    }

    fn on_new_scan(
        engine_execution_context: Arc<EngineExecutionContext>,
        data_type: DataTypeView,
    ) {
        // TODO: Push this into a converter perhaps, although gets tricky with args
        let scan_filter_parameters = match data_type {
            DataTypeView::I8 => ScanFilterParameters::new(None, DataType::I8()),
            DataTypeView::U8 => ScanFilterParameters::new(None, DataType::U8()),
            DataTypeView::I16 => ScanFilterParameters::new(None, DataType::I16(Endian::Little)),
            DataTypeView::I16be => ScanFilterParameters::new(None, DataType::I16(Endian::Big)),
            DataTypeView::U16 => ScanFilterParameters::new(None, DataType::U16(Endian::Little)),
            DataTypeView::U16be => ScanFilterParameters::new(None, DataType::U16(Endian::Big)),
            DataTypeView::I32 => ScanFilterParameters::new(None, DataType::I32(Endian::Little)),
            DataTypeView::I32be => ScanFilterParameters::new(None, DataType::I32(Endian::Big)),
            DataTypeView::U32 => ScanFilterParameters::new(None, DataType::U32(Endian::Little)),
            DataTypeView::U32be => ScanFilterParameters::new(None, DataType::U32(Endian::Big)),
            DataTypeView::I64 => ScanFilterParameters::new(None, DataType::I64(Endian::Little)),
            DataTypeView::I64be => ScanFilterParameters::new(None, DataType::I64(Endian::Big)),
            DataTypeView::U64 => ScanFilterParameters::new(None, DataType::U64(Endian::Little)),
            DataTypeView::U64be => ScanFilterParameters::new(None, DataType::U64(Endian::Big)),
            DataTypeView::F32 => ScanFilterParameters::new(None, DataType::F32(Endian::Little)),
            DataTypeView::F32be => ScanFilterParameters::new(None, DataType::F32(Endian::Big)),
            DataTypeView::F64 => ScanFilterParameters::new(None, DataType::F64(Endian::Little)),
            DataTypeView::F64be => ScanFilterParameters::new(None, DataType::F64(Endian::Big)),
            DataTypeView::Bytes => ScanFilterParameters::new(None, DataType::Bytes(0)), // TODO
            DataTypeView::String => ScanFilterParameters::new(None, DataType::Bytes(0)), // TODO
            DataTypeView::Bitfield => ScanFilterParameters::new(None, DataType::BitField(0)), // TODO
        };

        let scan_new_request = ScanNewRequest {
            scan_filter_parameters: vec![scan_filter_parameters],
            scan_all_primitives: false,
        };

        scan_new_request.send(&engine_execution_context, |_scan_new_response| {});
    }

    fn on_start_scan(
        engine_execution_context: Arc<EngineExecutionContext>,
        scan_constraint: ScanConstraintTypeView,
        scan_value: SharedString,
    ) {
        let scan_value = AnonymousValue::new(&scan_value.to_string());
        let scan_hybrid_request = ScanHybridRequest {
            scan_value: Some(scan_value),
            compare_type: ScanConstraintConverter::new().convert_from_view_data(&scan_constraint),
        };

        scan_hybrid_request.send(&engine_execution_context, |_scan_hybrid_response| {});
    }

    fn on_collect_values(engine_execution_context: Arc<EngineExecutionContext>) {
        let collect_values_request = ScanCollectValuesRequest {};

        collect_values_request.send(&engine_execution_context, |_scan_collect_values_response| {});
    }
}
